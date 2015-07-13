use super::graph::DirectedGraph;
use super::graph::Edge;
use std::collections::VecDeque;
use std::collections::HashMap;
use rand;
use rand::Rng;
use super::labels::Symbol;
use super::labels::SymbolSet;
use super::labels::SearchNode;
use super::labels::SearchEdge;

pub struct Rule<T: Symbol, S: Symbol, U: SymbolSet<T>, V: SymbolSet<S>> {
    start: DirectedGraph<U, V>,
    result: DirectedGraph<T, S>,
    start_to_res: HashMap<usize, usize>,
    res_to_start: HashMap<usize, usize>,
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
        for start_index in 0..subgraph.len() {
            if let Some(result_index) = self.start_to_res.get(&start_index) {
                {
                    let new_node = &self.result.get_node(*result_index);
                    let old_node = graph.mut_node(subgraph[start_index]);
                    if old_node.label != new_node.label {
                        old_node.label = new_node.label.clone();
                    }
                }
                for edge in self.start.get_node(start_index).to.iter() {
                    let edge_target = self.start_to_res.get(&edge.to);
                    if edge_target.is_none() ||
                       self.result.get_node(*edge_target.unwrap()).from.iter()
                           .all(|&from| from != self.start_to_res[&edge.from]) {
                        graph.remove_edge(subgraph[edge.from], subgraph[edge.to]);
                    }
                }
            }
            else {
                graph.remove_node(subgraph[start_index]);
            }
        }
    }

    fn add_new(&self, graph: &mut DirectedGraph<T, S>, subgraph: &[usize]) {
        let mut node_indexes = Vec::new();

        for result_index in 0..self.result.nodes() {
            let res_node = self.result.get_node(result_index);
            let graph_index =
                if let Some(start_index) = self.res_to_start.get(&result_index) {
                    subgraph[*start_index]
                }
                else {
                    graph.push_node(res_node.label.clone());
                    graph.nodes() - 1
                };
            node_indexes.push(graph_index);
        }

        for index in 0..self.result.nodes() {
            let res_node = self.result.get_node(index);
            let start_node = self.res_to_start.get(&index);
            for edge in res_node.to.iter() {
                //If edge doesn't exist in start graph
                //Get the equivalent of edge.to in the target graph
                //And create edge from graph_index to the eq of edge.to
                let start_target = self.res_to_start.get(&edge.to);
                if start_node.is_none() || start_target.is_none() ||
                   self.start.get_node(*start_node.unwrap()).to.iter()
                        .all(|e| e.to != *start_target.unwrap()) {
                    graph.add_edge(node_indexes[index], node_indexes[edge.to], edge.label.clone(), true);
                }
                //If it does exist, and has a different label
                //Change its label to the result graph version
                else {
                    let mut graph_node = graph.mut_node(node_indexes[index]);
                    let graph_edge = graph_node.to.iter_mut().find(|e| e.to == node_indexes[edge.to]).unwrap();
                    if graph_edge.label != edge.label {
                        graph_edge.label = edge.label.clone();
                    }
                }
            }
            for edge_index in res_node.from.iter() {
                let edge = self.result.get_node(*edge_index).to.iter().find(|e| e.to == index).unwrap();
                let start_origin = self.res_to_start.get(&edge.from);
                if start_node.is_none() || start_origin.is_none() ||
                   self.start.get_node(*start_node.unwrap()).from.iter()
                        .all(|&i| i != *start_origin.unwrap()) {
                    graph.add_edge(node_indexes[edge.from], node_indexes[index], edge.label.clone(), true);
                }
                else {
                    let mut graph_origin = graph.mut_node(node_indexes[edge.from]);
                    let graph_edge = graph_origin.to.iter_mut().find(|e| e.to == node_indexes[edge.to]).unwrap();
                    if graph_edge.label != edge.label {
                        graph_edge.label = edge.label.clone();
                    }
                }
            }
        }
    }
}
