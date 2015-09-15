use super::graph::DirectedGraph;
use super::labels::Symbol;

pub struct GraphContract {
    pub is_acyclic: bool,
    pub max_edges_per_node: Option<u32>,
    pub is_connected: bool,
}

impl GraphContract {
    pub fn holds_for<V: Symbol, E: Symbol>(&self, graph: &DirectedGraph<V, E>) -> bool {
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

    pub fn check_if_acyclic<V: Symbol, E: Symbol>(graph: &DirectedGraph<V, E>) -> bool {
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

    pub fn check_max_edges<V: Symbol, E: Symbol>(graph: &DirectedGraph<V, E>, max: usize) -> bool {
        for i in 0..graph.nodes() {
            if graph.get_node(i).to.len() > max {
                return false;
            }
        }
        true
    }

    //All relevant graphs are expected to be connected
    pub fn check_if_connected<V: Symbol, E: Symbol>(graph: &DirectedGraph<V, E>) -> bool {
        Self::bfs(graph, 0) == graph.nodes()
    }

    fn bfs<V: Symbol, E: Symbol>(graph: &DirectedGraph<V, E>, start: usize) -> usize {
        let mut last = vec!(start);
        let mut marked = Vec::new();
        marked.push(start);

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
                        //println!("{}", marked.len());
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
