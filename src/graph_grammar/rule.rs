use super::graph::DirectedGraph;
use super::graph::Edge;
//use std::collections::linked_list::LinkedList;
use std::collections::VecDeque;
use rand;
use rand::Rng;
use super::labels::Symbol;
use super::labels::SymbolSet;
use super::labels::SearchNode;
use super::labels::SearchEdge;

pub struct Rule<T: Symbol, S: Symbol, U: SymbolSet<T>, V: SymbolSet<S>> {
    start: DirectedGraph<U, V>,
    result: DirectedGraph<T, S>,
    root: usize,
    enforce_direction: bool,
}

impl<T: Symbol, S: Symbol, U: SymbolSet<T>, V: SymbolSet<S>> Rule<T, S, U, V> {
    pub fn apply_to(&self, graph: &mut DirectedGraph<T, S>) -> bool {
        panic!("");
    }

    pub fn find_subgraphs(&self, graph: &DirectedGraph<T, S>) -> Vec<Vec<usize>> {
        let not_found = self.start.nodes();
        let mut ret = Vec::new();
        for i in 0..graph.nodes() {
            let mut try = Vec::new();
            if self.start.get_node(self.root).label.is_superset_of(&graph.get_node(i).label) {
                for i in 0..self.start.nodes() {
                    try.push(not_found);
                }
                try[self.root] = i;
                ret.push(try);
            }
        }
        let mut edges_left = VecDeque::with_capacity(10);
        let mut edges_explored = Vec::new();
        for edge in self.start.get_node(self.root).to.iter() {
            edges_left.push_back(edge);
        }

        while edges_left.len() > 0 {
            let edge = edges_left.pop_front().unwrap();
            let mut new_subgraphs = Vec::new();

            for subgraph in ret.iter_mut() {
                let matching_edges = self.edge_matches(edge, subgraph[edge.from], graph);
                let to = if matching_edges.len() > 0 { matching_edges[0].to } else { not_found };
                if subgraph[edge.to] == not_found {
                    subgraph[edge.to] = to;
                }
                else if subgraph[edge.to] != to {
                    subgraph[edge.to] = not_found;
                }

                if matching_edges.len() > 1 {
                    for e in matching_edges[1..].iter() {
                        let mut new = subgraph.clone();
                        if new[edge.to] == not_found {
                            new[edge.to] = e.to;
                        }
                        else if new[edge.to] != to {
                            new[edge.to] = not_found;
                        }
                        new_subgraphs.push(new);
                    }
                }
            }
            ret.append(&mut new_subgraphs);

            for index in (ret.len()..0) {
                if ret[index][edge.to] == not_found {
                    ret.swap_remove(index);
                }
            }
            if ret.len() > 0 {
                for new_edge in self.start.get_node(edge.to).to.iter() {
                    if edges_explored.iter().all(|e| !new_edge.matches(*e)) {
                        edges_left.push_back(new_edge);
                    }
                }
            }
            edges_explored.push((edge.from, edge.to));
        }
        return ret;
    }

    fn edge_matches<'a>(&self,
                        rule_edge: &Edge<V>,
                        graph_start: usize,
                        graph: &'a DirectedGraph<T, S>) -> Vec<&'a Edge<S>> {
        let mut ret = Vec::new();
        for edge in graph.get_node(graph_start).to.iter() {
            if rule_edge.label.is_superset_of(&edge.label)
               && self.start.get_node(rule_edge.to).label.is_superset_of(&graph.get_node(edge.to).label) {
                ret.push(edge);
            }
        }
        return ret;
    }

    fn alter_old(&self, graph: &mut DirectedGraph<T, S>, subgraph: &[usize]) {

    }

    fn add_new(&self, graph: &mut DirectedGraph<T, S>, subgraph: &[usize]) {

    }
}
