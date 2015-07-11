use super::graph::DirectedGraph;
use super::graph::Edge;
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
        let empty = self.start.nodes();
        let mut ret = Vec::new();
        for i in 0..graph.nodes() {
            if let Some(vec) = self.initial_node(i, graph) {
                ret.push(vec);
            }
        }
        let mut edges_left = VecDeque::with_capacity(10);
        let mut edges_explored = Vec::new();
        for edge in self.start.get_node(self.root).to.iter() {
            edges_left.push_back(edge);
        }

        while edges_left.len() > 0 && ret.len() > 0 {
            let edge = edges_left.pop_front().unwrap();
            let mut new_subgraphs = Vec::new();
            for subgraph in ret.iter_mut() {
                let forks = self.update_subgraph(edge, subgraph, graph);
                for fork in forks {
                    new_subgraphs.push(fork);
                }
            }
            for s in new_subgraphs {
                ret.push(s);
            }
            for index in (ret.len()..0) {
                if ret[index][edge.to] == empty {
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

    fn initial_node<'a>(&self, index: usize, graph: &'a DirectedGraph<T, S>) -> Option<Vec<usize>> {
        let empty = self.start.nodes();
        let mut try = Vec::new();
        if self.start.get_node(self.root).label.is_superset_of(&graph.get_node(index).label) {
            for _ in 0..self.start.nodes() {
                try.push(empty);
            }
            try[self.root] = index;
        }
        if try.len() > 0 { Some(try) } else { None }
    }

    fn update_subgraph(&self, rule_edge: &Edge<V>, subgraph: &mut Vec<usize>, graph: &DirectedGraph<T, S>) -> Vec<Vec<usize>> {
        let empty = self.start.nodes();
        let mut new_subgraphs = Vec::new();
        let matching_edges = self.edge_matches(rule_edge, subgraph[rule_edge.from], graph);
        let to = if matching_edges.len() > 0 { matching_edges[0].to } else { empty };
        if subgraph[rule_edge.to] == empty {
            subgraph[rule_edge.to] = to;
        }
        else if subgraph[rule_edge.to] != to {
            subgraph[rule_edge.to] = empty;
        }

        if matching_edges.len() > 1 {
            for e in matching_edges[1..].iter() {
                let mut new = subgraph.clone();
                if new[rule_edge.to] == empty {
                    new[rule_edge.to] = e.to;
                }
                else if new[rule_edge.to] != to {
                    new[rule_edge.to] = empty;
                }
                new_subgraphs.push(new);
            }
        }
        return new_subgraphs;
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
