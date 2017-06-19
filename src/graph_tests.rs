use petgraph::graph::{Graph, NodeIndex, node_index};
use petgraph::visit::Walker;

use BfsWithDepth;

lazy_static! {
static ref GRAPH_DEPTH_4: Graph<(), usize> = Graph::<(), usize>::from_edges(&[
    (0, 1), (0, 2), (0, 3),
    (1, 4), (1, 5), (5, 6),
    (3, 7), (3, 8)
]);

/// Note: for firected graphs, neighbors are returned in reverse order
/// of addition.
static ref GRAPH_DEPTH_4_BFS_WITH_DEPTH: Vec<(NodeIndex, usize)> = vec![
	(node_index(0),0),
	(node_index(3),1),
	(node_index(2),1),
	(node_index(1),1),
	(node_index(8),2),
	(node_index(7),2),
	(node_index(5),2),
	(node_index(4),2),
	(node_index(6),3)];
}

#[test]
pub fn bfs_with_depth_test() {
    let visits: Vec<_> = BfsWithDepth::new(&*GRAPH_DEPTH_4, node_index(0))
        .iter(&*GRAPH_DEPTH_4)
        .collect();
    assert_eq!(*GRAPH_DEPTH_4_BFS_WITH_DEPTH, visits);
}
