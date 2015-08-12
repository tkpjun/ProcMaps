use super::graph::DirectedGraph;
use super::labels::Symbol;
use std::marker::PhantomData;

pub struct GraphContract {
    pub is_acyclic: bool,
    pub max_edges_per_node: Option<u32>,
}

impl GraphContract {
    pub fn holds_for<T: Symbol, S: Symbol>(&self, graph: &DirectedGraph<T, S>) -> bool {
        let mut ret = true;
        if self.is_acyclic {
            ret = Self::check_if_acyclic(graph);
        }
        if let Some(max) = self.max_edges_per_node {
            ret = Self::check_max_edges(graph);
        }
        ret
    }

    pub fn check_if_acyclic<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>) -> bool {
        //attempt to use topological sorting on cloned graph
        unimplemented!();
    }

    pub fn check_max_edges<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>) -> bool {
        unimplemented!();
    }

    //All relevant graphs are expected to be connected
    pub fn check_if_connected<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>) -> bool {
        unimplemented!();
    }
}

impl Default for GraphContract {
    fn default() -> GraphContract {
        GraphContract{ is_acyclic: false, max_edges_per_node: None }
    }
}
