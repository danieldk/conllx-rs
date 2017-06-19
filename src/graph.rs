use std::collections::VecDeque;
use std::mem;

pub use petgraph::visit::{GraphRef, IntoNeighbors, VisitMap, Visitable, Walker};

pub struct BfsWithDepth<N, VM> {
    cur_stack: VecDeque<N>,
    next_stack: VecDeque<N>,
    discovered: VM,
    depth: usize,
}

impl<N, VM> BfsWithDepth<N, VM>
where
    N: Copy + PartialEq,
    VM: VisitMap<N>,
{
    pub fn new<G>(graph: G, start: N) -> Self
    where
        G: GraphRef + Visitable<NodeId = N, Map = VM>,
    {
        let mut discovered = graph.visit_map();
        discovered.visit(start);
        let mut cur_stack = VecDeque::new();
        cur_stack.push_back(start);

        BfsWithDepth {
            cur_stack,
            next_stack: VecDeque::new(),
            discovered,
            depth: 0,
        }
    }

    pub fn next<G>(&mut self, graph: G) -> Option<(N, usize)>
    where
        G: IntoNeighbors<NodeId = N>,
    {
        if self.cur_stack.is_empty() && !self.next_stack.is_empty() {
            mem::swap(&mut self.cur_stack, &mut self.next_stack);
            self.depth += 1;
        }

        while let Some(node) = self.cur_stack.pop_front() {
            for succ in graph.neighbors(node) {
                if self.discovered.visit(succ) {
                    self.next_stack.push_back(succ)
                }
            }

            return Some((node, self.depth));
        }

        None
    }
}

impl<G> Walker<G> for BfsWithDepth<G::NodeId, G::Map>
where
    G: IntoNeighbors + Visitable,
{
    type Item = (G::NodeId, usize);

    fn walk_next(&mut self, context: G) -> Option<Self::Item> {
        self.next(context)
    }
}
