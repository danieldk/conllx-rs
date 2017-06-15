use petgraph::graph::{NodeIndex, node_index};

use {Deprojectivize, HeadProjectivizer, Projectivize, Sentence, TokenBuilder, sentence_to_graph,
     non_projective_edges};

lazy_static! {
static ref PROJECTIVE_SENTENCES: Vec<Sentence> =
    vec![
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Für")
                .head(4)
                .head_rel("PP|OBJA")
                .token(),
            TokenBuilder::new()
                .form("diese")
                .head(3)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Behauptung")
                .head(1)
                .head_rel("PN")
                .token(),
            TokenBuilder::new()
                .form("hat")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("Beckmeyer")
                .head(4)
                .head_rel("SUBJ")
                .token(),
            TokenBuilder::new()
                .form("bisher")
                .head(9)
                .head_rel("ADV")
                .token(),
            TokenBuilder::new()
                .form("keinen")
                .head(8)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Nachweis")
                .head(9)
                .head_rel("OBJA")
                .token(),
            TokenBuilder::new()
                .form("geliefert")
                .head(4)
                .head_rel("AUX")
                .token(),
            TokenBuilder::new()
                .form(".")
                .head(9)
                .head_rel("-PUNCT-")
                .token(),
        ]),

        Sentence::new(vec![
            TokenBuilder::new()
                .form("Auch")
                .head(2)
                .head_rel("ADV")
                .token(),
            TokenBuilder::new()
                .form("für")
                .head(5)
                .head_rel("PP|PN")
                .token(),
            TokenBuilder::new()
                .form("Rumänien")
                .head(2)
                .head_rel("PN")
                .token(),
            TokenBuilder::new()
                .form("selbst")
                .head(3)
                .head_rel("ADV")
                .token(),
            TokenBuilder::new()
                .form("ist")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("der")
                .head(7)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Papst-Besuch")
                .head(5)
                .head_rel("SUBJ")
                .token(),
            TokenBuilder::new()
                .form("von")
                .head(5)
                .head_rel("PRED")
                .token(),
            TokenBuilder::new()
                .form("großer")
                .head(10)
                .head_rel("ATTR")
                .token(),
            TokenBuilder::new()
                .form("Bedeutung")
                .head(8)
                .head_rel("PN")
                .token(),
            TokenBuilder::new()
                .form(".")
                .head(10)
                .head_rel("-PUNCT-")
                .token(),
        ]),

        // From Nivre & Nilsson, 2005
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Z")
                .head(3)
                .head_rel("AuxP|Sb")
                .token(),
            TokenBuilder::new()
                .form("nich")
                .head(1)
                .head_rel("Atr")
                .token(),
            TokenBuilder::new()
                .form("je")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("jen")
                .head(5)
                .head_rel("AuxZ")
                .token(),
            TokenBuilder::new()
                .form("jedna")
                .head(3)
                .head_rel("Sb")
                .token(),
            TokenBuilder::new()
                .form("na")
                .head(3)
                .head_rel("AuxP")
                .token(),
            TokenBuilder::new()
                .form("kvalitu")
                .head(6)
                .head_rel("Adv")
                .token(),
            TokenBuilder::new()
                .form(".")
                .head(3)
                .head_rel("AuxZ")
                .token(),
        ]),

        // Two non-projectivite edges.
        Sentence::new(vec![
            TokenBuilder::new()
                .form("a")
                .head(4)
                .head_rel("a")
                .token(),
            TokenBuilder::new()
                .form("b")
                .head(4)
                .head_rel("b")
                .token(),
            TokenBuilder::new()
                .form("c")
                .head(4)
                .head_rel("c|a")
                .token(),
            TokenBuilder::new()
                .form("d")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("e")
                .head(4)
                .head_rel("e|g")
                .token(),
            TokenBuilder::new()
                .form("f")
                .head(4)
                .head_rel("f")
                .token(),
            TokenBuilder::new()
                .form("g")
                .head(4)
                .head_rel("g")
                .token(),
        ]),
    ];

static ref NON_PROJECTIVE_SENTENCES: Vec<Sentence> =
    vec![
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Für")
                .head(8)
                .head_rel("PP")
                .token(),
            TokenBuilder::new()
                .form("diese")
                .head(3)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Behauptung")
                .head(1)
                .head_rel("PN")
                .token(),
            TokenBuilder::new()
                .form("hat")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("Beckmeyer")
                .head(4)
                .head_rel("SUBJ")
                .token(),
            TokenBuilder::new()
                .form("bisher")
                .head(9)
                .head_rel("ADV")
                .token(),
            TokenBuilder::new()
                .form("keinen")
                .head(8)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Nachweis")
                .head(9)
                .head_rel("OBJA")
                .token(),
            TokenBuilder::new()
                .form("geliefert")
                .head(4)
                .head_rel("AUX")
                .token(),
            TokenBuilder::new()
                .form(".")
                .head(9)
                .head_rel("-PUNCT-")
                .token(),
        ]),

        Sentence::new(vec![
            TokenBuilder::new()
                .form("Auch")
                .head(2)
                .head_rel("ADV")
                .token(),
            TokenBuilder::new()
                .form("für")
                .head(10)
                .head_rel("PP")
                .token(),
            TokenBuilder::new()
                .form("Rumänien")
                .head(2)
                .head_rel("PN")
                .token(),
            TokenBuilder::new()
                .form("selbst")
                .head(3)
                .head_rel("ADV")
                .token(),
            TokenBuilder::new()
                .form("ist")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("der")
                .head(7)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Papst-Besuch")
                .head(5)
                .head_rel("SUBJ")
                .token(),
            TokenBuilder::new()
                .form("von")
                .head(5)
                .head_rel("PRED")
                .token(),
            TokenBuilder::new()
                .form("großer")
                .head(10)
                .head_rel("ATTR")
                .token(),
            TokenBuilder::new()
                .form("Bedeutung")
                .head(8)
                .head_rel("PN")
                .token(),
            TokenBuilder::new()
                .form(".")
                .head(10)
                .head_rel("-PUNCT-")
                .token(),
        ]),

        // From Nivre & Nilsson, 2005
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Z")
                .head(5)
                .head_rel("AuxP")
                .token(),
            TokenBuilder::new()
                .form("nich")
                .head(1)
                .head_rel("Atr")
                .token(),
            TokenBuilder::new()
                .form("je")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("jen")
                .head(5)
                .head_rel("AuxZ")
                .token(),
            TokenBuilder::new()
                .form("jedna")
                .head(3)
                .head_rel("Sb")
                .token(),
            TokenBuilder::new()
                .form("na")
                .head(3)
                .head_rel("AuxP")
                .token(),
            TokenBuilder::new()
                .form("kvalitu")
                .head(6)
                .head_rel("Adv")
                .token(),
            TokenBuilder::new()
                .form(".")
                .head(3)
                .head_rel("AuxZ")
                .token(),
        ]),

        // Two non-projectivite edges.
        Sentence::new(vec![
            TokenBuilder::new()
                .form("a")
                .head(4)
                .head_rel("a")
                .token(),
            TokenBuilder::new()
                .form("b")
                .head(4)
                .head_rel("b")
                .token(),
            TokenBuilder::new()
                .form("c")
                .head(1)
                .head_rel("c")
                .token(),
            TokenBuilder::new()
                .form("d")
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("e")
                .head(7)
                .head_rel("e")
                .token(),
            TokenBuilder::new()
                .form("f")
                .head(4)
                .head_rel("f")
                .token(),
            TokenBuilder::new()
                .form("g")
                .head(4)
                .head_rel("g")
                .token(),
        ]),
    ];

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

#[test]
fn deprojectivize_test() {
    let projectivizer = HeadProjectivizer::new();
    let non_projective: Vec<_> = PROJECTIVE_SENTENCES
        .iter()
        .map(|s| {
            projectivizer
                .deprojectivize(s)
                .expect("Cannot deprojectivize sentence")
        })
        .collect();


    assert_eq!(*NON_PROJECTIVE_SENTENCES, non_projective);
}

#[test]
fn non_projective_test() {
    let test_edges = sent_non_projective_edges(&NON_PROJECTIVE_SENTENCES);
    assert_eq!(*NON_PROJECTIVE_EDGES, test_edges);
}

#[test]
fn projectivize_test() {
    let projectivizer = HeadProjectivizer::new();
    let projective: Vec<_> = NON_PROJECTIVE_SENTENCES
        .iter()
        .map(|s| {
            projectivizer
                .projectivize(s)
                .expect("Cannot projectivize sentence")
        })
        .collect();


    assert_eq!(*PROJECTIVE_SENTENCES, projective);
}
