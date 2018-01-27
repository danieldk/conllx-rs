use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};

use itertools::Itertools;
use petgraph::{Directed, Direction, Graph};
use petgraph::graph::{node_index, EdgeIndex, NodeIndex};
use petgraph::visit::{Bfs, EdgeRef, NodeFiltered, Walker};

use BfsWithDepth;
use GraphError;
use Sentence;

pub trait Deprojectivize {
    fn deprojectivize(&self, sentence: &Sentence) -> Result<Sentence, GraphError>;
}

pub trait Projectivize {
    fn projectivize(&self, sentence: &Sentence) -> Result<Sentence, GraphError>;
}

/// A projectivizer using the 'head' marking strategy. See: *Pseudo-Projective
/// Dependency Parsing*, Nivre and Nilsson, 2005.
pub struct HeadProjectivizer;

impl HeadProjectivizer {
    pub fn new() -> Self {
        HeadProjectivizer {}
    }

    /// Deprojectivize the next node in the array of lifted nodes.
    ///
    /// Returns the index of the node that was lifted.
    fn deprojectivize_next(
        &self,
        graph: &mut Graph<(), String, Directed>,
        lifted_sorted: &[NodeIndex],
        head_labels: &HashMap<NodeIndex, String>,
    ) -> Option<usize> {
        for (idx, lifted_node) in lifted_sorted.iter().enumerate() {
            let pref_head_rel = head_labels
                .get(lifted_node)
                .expect("Lifted node without preferred head relation");

            let head_edge = graph
                .first_edge(*lifted_node, Direction::Incoming)
                .expect("Lifted node without an incoming edge");
            let (cur_head, _) = graph
                .edge_endpoints(head_edge)
                .expect("Endpoints of lifted edge could not be found");

            if let Some(new_head) =
                self.search_attachment_point(&graph, cur_head, *lifted_node, pref_head_rel)
            {
                let head_rel = graph
                    .remove_edge(head_edge)
                    .expect("Lifted edge to be removed could not be found");
                graph.add_edge(new_head, *lifted_node, head_rel);
                return Some(idx);
            }
        }

        None
    }

    /// Find the correct attachment point for the lifted token/node.
    fn search_attachment_point(
        &self,
        graph: &Graph<(), String, Directed>,
        cur_head: NodeIndex,
        lifted_node: NodeIndex,
        pref_head_rel: &str,
    ) -> Option<NodeIndex> {
        // We are looking for a token dominated by cur_head to attach
        // lifted_node to. This token should:
        //
        // 1. Be attached to its head using pref_head_rel.
        // 2. Not be lifted_node itself or any of its decendants.
        // 3. As high in the tree as possible.
        //
        // From the set of candidates, we pick the token that is the
        // closest to the current head.

        // Requirement (2): use a view of the graph that excludes
        // to avoid attachment to the lifted_node or any of its children.
        let graph_without_lifted = NodeFiltered::from_fn(graph, |n| n != lifted_node);


        // Requirement (3): process the dependency tree by increasing depth
        // until the reattachment token is found.
        for (_, nodes) in &BfsWithDepth::new(&graph_without_lifted, node_index(0))
            .iter(&graph_without_lifted)
            .skip(1)
            .group_by(|&(_, depth)| depth)
        {
            // Requirement (1): Only retain edges with the preferred relation.
            let level_candidates = nodes.map(|(node, _)| node).filter(|&node| {
                let edge = match graph.first_edge(node, Direction::Incoming) {
                    Some(edge) => edge,
                    None => return false,
                };

                graph[edge] == pref_head_rel
            });

            // When there are multiple candidates, return the token closes to the head.
            let min_candidate = level_candidates.min_by_key(|&node| {
                max(node.index(), cur_head.index()) - min(node.index(), cur_head.index())
            });

            if min_candidate.is_some() {
                return min_candidate;
            }
        }

        None
    }

    /// Lift the edge identified by `edge_idx`. This will reattach the edge
    /// to the parent of the head. If this was the first lifting operation,
    /// the dependency relation of the original head is added to the dependency
    /// relation (following the head-strategy).
    fn lift(
        &self,
        graph: &mut Graph<(), String, Directed>,
        lifted: &mut HashSet<NodeIndex>,
        edge_idx: EdgeIndex,
    ) {
        let (source, target) = graph
            .edge_endpoints(edge_idx)
            .expect("lift() called with invalid index");
        let parent_edge = graph
            .first_edge(source, Direction::Incoming)
            .expect("Cannot find incoming edge of the to-be lifted node");
        let parent_rel = graph[parent_edge].clone();
        let (parent, _) = graph
            .edge_endpoints(parent_edge)
            .expect("Cannot find endpoints of to-be lifted edge");

        let rel = graph
            .remove_edge(edge_idx)
            .expect("Cannot remove edge to-be lifted");

        if lifted.contains(&target) {
            graph.add_edge(parent, target, rel);
        } else {
            graph.add_edge(parent, target, format!("{}|{}", rel, parent_rel));
            lifted.insert(target);
        }
    }

    /// Prepare for deprojectivizing: remove head annotations from lifted
    /// relations. Return the transformed graph + indices of lifted nodes
    /// and their head labels.
    fn prepare_deproj(
        &self,
        graph: &Graph<(), String, Directed>,
    ) -> (Graph<(), String, Directed>, HashMap<NodeIndex, String>) {
        let mut pref_head_labels = HashMap::new();

        let prepared_graph = graph.map(
            |_, &node_val| node_val,
            |edge_idx, edge_val| {
                let sep_idx = match edge_val.find('|') {
                    Some(idx) => idx,
                    None => return edge_val.clone(),
                };

                let (_, dep) = graph
                    .edge_endpoints(edge_idx)
                    .expect("Cannot lookup edge endpoints");

                pref_head_labels.insert(dep, edge_val[sep_idx + 1..].to_owned());

                edge_val[..sep_idx].to_owned()
            },
        );

        (prepared_graph, pref_head_labels)
    }
}

impl Projectivize for HeadProjectivizer {
    fn projectivize(&self, sentence: &Sentence) -> Result<Sentence, GraphError> {
        let mut graph = sentence_to_graph(sentence)?;
        let mut lifted = HashSet::new();

        // Lift non-projective edges until there are no non-projective
        // edges left.
        loop {
            let np_edges = non_projective_edges(&graph);
            if np_edges.is_empty() {
                break;
            }

            self.lift(&mut graph, &mut lifted, np_edges[0]);
        }

        // The graph is now a projective tree. Update the dependency relations
        // in the sentence to correspond to the graph.
        Ok(update_sentence(&graph, sentence))
    }
}

impl Deprojectivize for HeadProjectivizer {
    fn deprojectivize(&self, sentence: &Sentence) -> Result<Sentence, GraphError> {
        let graph = sentence_to_graph(sentence)?;

        // Find nodes and corresponding edges that are lifted and remove
        // head labels from dependency relations.
        let (mut graph, head_labels) = self.prepare_deproj(&graph);
        if head_labels.is_empty() {
            return Ok(sentence.clone());
        }

        // Get and sort lifted tokens by increasing depth.
        let mut lifted_sorted = Vec::new();
        let mut bfs = Bfs::new(&graph, node_index(0));
        while let Some(node) = bfs.next(&graph) {
            if head_labels.get(&node).is_some() {
                lifted_sorted.push(node);
            }
        }

        // Deprojectivize the graph, re-attaching one token at a time,
        // with the preference of a token that is not deep in the tree.
        loop {
            match self.deprojectivize_next(&mut graph, &lifted_sorted, &head_labels) {
                Some(idx) => lifted_sorted.remove(idx),
                None => break,
            };
        }

        Ok(update_sentence(&graph, sentence))
    }
}

pub fn sentence_to_graph(sentence: &Sentence) -> Result<Graph<(), String, Directed>, GraphError> {
    let mut edges = Vec::with_capacity(sentence.len() + 1);
    for (idx, token) in sentence.iter().enumerate() {
        let (head, dependent) = match token.head() {
            Some(head) => (node_index(head), node_index(idx + 1)),
            None => continue,
        };

        let head_rel = match token.head_rel() {
            Some(head_rel) => head_rel,
            None => {
                return Err(
                    GraphError::IncompleteGraph {
                        value: format!(
                            "edge from {} to {} does not have a label",
                            head.index(),
                            dependent.index())
                    }
                )
            }
        };

        edges.push((head, dependent, head_rel.to_owned()))
    }

    Ok(Graph::<(), String, Directed>::from_edges(edges))
}

/// Returns non-projective edges in the graph, ordered by length.
pub fn non_projective_edges(graph: &Graph<(), String, Directed>) -> Vec<EdgeIndex> {
    let mut non_projective = Vec::new();

    for i in 0..graph.node_count() {
        let mut i_reachable = HashSet::new();
        let mut bfs = Bfs::new(&graph, node_index(i));
        while let Some(node) = bfs.next(&graph) {
            i_reachable.insert(node.index());
        }

        for edge in graph.edges(node_index(i)) {
            // An edge i -> k is projective, iff:
            //
            // i > j > k or i < j < k, and i ->* j
            for j in min(i, edge.target().index())..max(i, edge.target().index()) {
                if !i_reachable.contains(&j) {
                    non_projective.push(edge);
                    break;
                }
            }
        }
    }

    non_projective.sort_by(|a, b| {
        let a_len = max(a.source().index(), a.target().index()) -
            min(a.source().index(), a.target().index());
        let b_len = max(b.source().index(), b.target().index()) -
            min(b.source().index(), b.target().index());

        a_len.cmp(&b_len)
    });

    non_projective.iter().map(|eref| eref.id()).collect()
}

/// Update a sentence with dependency relations from a graph.
fn update_sentence(graph: &Graph<(), String, Directed>, sent: &Sentence) -> Sentence {
    let mut new_sent = sent.clone();
    {
        for edge_ref in graph.edge_references() {
            new_sent[edge_ref.target().index() - 1].set_head(Some(edge_ref.source().index()));
            new_sent[edge_ref.target().index() - 1].set_head_rel(Some(edge_ref.weight().clone()));
        }
    }

    new_sent
}

#[cfg(test)]
mod tests {
    use petgraph::graph::{node_index, NodeIndex};

    use {non_projective_edges, sentence_to_graph, Deprojectivize, HeadProjectivizer, Projectivize,
         Sentence};
    use tests::read_sentences;

    lazy_static! {
        static ref NON_PROJECTIVE_EDGES: Vec<Vec<(NodeIndex, NodeIndex)>> = vec![
        vec![(node_index(8), node_index(1))],
        vec![(node_index(10), node_index(2))],
        vec![(node_index(5), node_index(1))],
        vec![(node_index(1), node_index(3)), (node_index(7), node_index(5))],
        ];
    }

    fn sent_non_projective_edges(sents: &[Sentence]) -> Vec<Vec<(NodeIndex, NodeIndex)>> {
        let mut np_edges = Vec::new();

        for sent in sents {
            let graph = sentence_to_graph(sent).unwrap();
            let np: Vec<_> = non_projective_edges(&graph)
                .iter()
                .map(|idx| graph.edge_endpoints(*idx).unwrap())
                .collect();
            np_edges.push(np);
        }

        np_edges
    }

    static PROJECTIVE_SENTENCES_FILENAME: &str = "testdata/projective.conll";

    static NONPROJECTIVE_SENTENCES_FILENAME: &str = "testdata/nonprojective.conll";

    #[test]
    fn deprojectivize_test() {
        let projectivizer = HeadProjectivizer::new();
        let non_projective: Vec<_> = read_sentences(PROJECTIVE_SENTENCES_FILENAME)
            .iter()
            .map(|s| {
                projectivizer
                    .deprojectivize(s)
                    .expect("Cannot deprojectivize sentence")
            })
            .collect();


        assert_eq!(
            read_sentences(NONPROJECTIVE_SENTENCES_FILENAME),
            non_projective
        );
    }

    #[test]
    fn non_projective_test() {
        let test_edges =
            sent_non_projective_edges(&read_sentences(NONPROJECTIVE_SENTENCES_FILENAME));
        assert_eq!(*NON_PROJECTIVE_EDGES, test_edges);
    }

    #[test]
    fn projectivize_test() {
        let projectivizer = HeadProjectivizer::new();
        let projective: Vec<_> = read_sentences(NONPROJECTIVE_SENTENCES_FILENAME)
            .iter()
            .map(|s| {
                projectivizer
                    .projectivize(s)
                    .expect("Cannot projectivize sentence")
            })
            .collect();

        assert_eq!(read_sentences(PROJECTIVE_SENTENCES_FILENAME), projective);
    }
}
