use super::graph::DirectedGraph;
use super::graph::DirectedAcyclicGraph;
use super::symbol::SymbolSet;
use rand;
use rand::Rng;

pub struct Rule<T: SymbolSet, S: SymbolSet> {
    start: DirectedAcyclicGraph<T, S>,
    result: DirectedGraph<T, S>,
    enforce_edge_dirs: bool,
}

impl<T: SymbolSet, S: SymbolSet> Rule<T, S> {
    pub fn apply_to(&self, graph: DirectedGraph<T, S>) {

    }
}
