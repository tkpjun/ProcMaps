use super::graph::DirectedGraph;
use super::graph::Edge;
use std::collections::VecDeque;
use std::collections::HashMap;
use super::labels::Symbol;
use super::labels::SymbolSet;

pub struct Rule<S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T>> {
    start: DirectedGraph<U, V>,
    result: DirectedGraph<S, T>,
    start_to_res: HashMap<usize, usize>,
    res_to_start: HashMap<usize, usize>,
    //enforce_direction: bool,
}

impl<S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T>> Rule<S, T, U, V> {

    pub fn new(start: DirectedGraph<U, V>, result: DirectedGraph<S, T>, start_to_res: HashMap<usize, usize>) -> Rule<S, T, U, V> {
        let res_to_start = start_to_res.iter().map(|(&k, &v)| (v, k)).collect();
        Rule{ start: start, result: result, start_to_res: start_to_res, res_to_start: res_to_start }
    }

    pub fn apply_to(&self, graph: &mut DirectedGraph<S, T>, subgraph: &[usize]) {
        let updated_subgraph = self.alter_old(graph, subgraph);
        self.add_new(graph, &updated_subgraph);
    }

    pub fn find_subgraphs(&self, graph: &DirectedGraph<S, T>) -> Vec<Vec<usize>> {
        let empty = self.start.nodes();
        let mut ret = Vec::new();
        for i in 0..graph.nodes() {
            if let Some(vec) = self.initial_node(i, graph) {
                ret.push(vec);
            }
        }
        let mut edges_left = VecDeque::with_capacity(10);
        let mut edges_explored = Vec::new();
        for edge in self.start.get_node(0).to.iter() {
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
                    else {
                        //start graph has a cycle in it: make sure that it's cyclical
                        //in the parameter graph as well
                        panic!("Searching for cyclical subgraphs is not supported!");
                    }
                }
            }
            edges_explored.push((edge.from, edge.to));
        }
        return ret;
    }

    fn initial_node<'a>(&self, index: usize, graph: &'a DirectedGraph<S, T>) -> Option<Vec<usize>> {
        let empty = self.start.nodes();
        let mut try = Vec::new();
        if self.start.get_node(0).label.is_superset_of(&graph.get_node(index).label) {
            for _ in 0..self.start.nodes() {
                try.push(empty);
            }
            try[0] = index;
        }
        if try.len() > 0 { Some(try) } else { None }
    }

    fn update_subgraph(&self, rule_edge: &Edge<V>, subgraph: &mut Vec<usize>, graph: &DirectedGraph<S, T>) -> Vec<Vec<usize>> {
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
                        graph: &'a DirectedGraph<S, T>) -> Vec<&'a Edge<T>> {
        let mut ret = Vec::new();
        for edge in graph.get_node(graph_start).to.iter() {
            if rule_edge.label.is_superset_of(&edge.label)
               && self.start.get_node(rule_edge.to).label.is_superset_of(&graph.get_node(edge.to).label) {
                ret.push(edge);
            }
        }
        return ret;
    }

    fn alter_old(&self, graph: &mut DirectedGraph<S, T>, subgraph: &[usize]) -> Vec<usize> {
        //update to remove all edges between nodes in start instead of just ones that point to
        //nodes that don't exist in result
        let mut new_sub = subgraph.iter().map(|&i| i).collect::<Vec<usize>>();
        for start_index in 0..subgraph.len() {
            if let Some(result_index) = self.start_to_res.get(&start_index) {
                {
                    let new_node = &self.result.get_node(*result_index).label;
                    let old_node = graph.mut_label(subgraph[start_index]);
                    if *old_node != *new_node {
                        *old_node = new_node.clone();
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
                if let Some(last_node_in_subgraph) = new_sub.iter_mut().find(|i| **i == graph.nodes()) {
                    *last_node_in_subgraph = subgraph[start_index];
                }
            }
        }
        new_sub
    }

    fn add_new(&self, graph: &mut DirectedGraph<S, T>, subgraph: &[usize]) {
        //update to add all edges in result, because all edges between nodes have been removed
        let node_indexes = self.build_result_subgraph(graph, subgraph);
        for index in 0..self.result.nodes() {
            let res_node = self.result.get_node(index);
            let start_node = self.res_to_start.get(&index);
            for edge in res_node.to.iter() {
                let start_target = self.res_to_start.get(&edge.to);
                if start_node.is_none() || start_target.is_none() ||
                   self.start.get_node(*start_node.unwrap()).to.iter()
                        .all(|e| e.to != *start_target.unwrap()) {
                    graph.add_edge(node_indexes[index], node_indexes[edge.to], edge.label.clone());
                }
                else {
                    let graph_edge = graph.mut_edge_label(node_indexes[index], node_indexes[edge.to]);
                    if *graph_edge != edge.label {
                        *graph_edge = edge.label.clone();
                    }
                }
            }
        }
    }

    fn build_result_subgraph(&self, graph: &mut DirectedGraph<S, T>, subgraph: &[usize]) -> Vec<usize>{
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
        return node_indexes;
    }
}
