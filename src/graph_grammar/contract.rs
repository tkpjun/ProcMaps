use super::graph::DirectedGraph;
use super::labels::Symbol;

pub struct GraphContract {
    pub is_acyclic: bool,
    pub max_edges_per_node: Option<u32>,
    pub is_connected: bool,
}

impl GraphContract {
    pub fn holds_for<T: Symbol, S: Symbol>(&self, graph: &DirectedGraph<T, S>) -> bool {
        if self.is_acyclic && Self::check_if_acyclic(graph) == false {
            return false;
        }
        if let Some(max) = self.max_edges_per_node {
            if Self::check_max_edges(graph, max as usize) == false {
                return false;
            }
        }
        if self.is_connected && Self::check_if_connected(graph) == false {
            return false;
        }
        true
    }

    pub fn check_if_acyclic<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>) -> bool {
        //attempt to use topological sorting on cloned graph
        let mut g = graph.clone();
        let mut found = true;
        while found {
            found = false;
            'inner: for i in 0..g.nodes() {
                if g.get_node(i).from.len() == 0 {
                    found = true;
                    g.remove_node(i);
                    break 'inner;
                }
            }
        }
        g.nodes() == 0
    }

    pub fn check_max_edges<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>, max: usize) -> bool {
        for i in 0..graph.nodes() {
            if graph.get_node(i).to.len() > max {
                return false;
            }
        }
        true
    }

    //All relevant graphs are expected to be connected
    pub fn check_if_connected<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>) -> bool {
        Self::bfs(graph, 0) == graph.nodes()
    }

    fn bfs<T: Symbol, S: Symbol>(graph: &DirectedGraph<T, S>, start: usize) -> usize {
        let mut last = vec!(start);
        let mut marked = Vec::new();

        let mut found = true;
        while found {
            found = false;
            let mut next = Vec::new();
            for i in last {
                for path in &graph.get_node(i).to {
                    if marked.iter().all(|a| *a != path.to) {
                        found = true;
                        marked.push(path.to);
                        next.push(path.to);
                    }
                }
            }
            last = next;
        }
        marked.len()
    }
}

impl Default for GraphContract {
    fn default() -> GraphContract {
        GraphContract{ is_acyclic: false, max_edges_per_node: None, is_connected: true }
    }
}
