use graph_grammar::graph::DirectedGraph;
use graph_grammar::rule::Rule;
use std::collections::HashMap;
use tests::DummyLabel::{self, A, B, C};

#[test]
fn find_subgraphs() {
    let graph = build_graph();
    let rule = build_rule();
    let subgraphs = rule.find_subgraphs(&graph);
    assert!(subgraphs.len() == 2);
    assert_eq!(format!("{:?}", subgraphs[0]), "([0, 1], None)");
}

#[test]
fn apply_rule() {
    let mut graph = build_graph();
    let rule = build_rule();
    let subgraphs = rule.find_subgraphs(&graph);
    rule.apply_to(&mut graph, &subgraphs[0].0, &subgraphs[0].1);
    /*assert_eq!(graph.to_string(),
    "1,2,-> A -> B(0-2),\n\
    -> A -> A(1-0),\n\
    0,-> C -> B(2-0),\n");*/
    let should_be = DirectedGraph::from_vec(&[A, A, C], &[(0, 2, B), (1, 0, A), (2, 0, B)]);
    assert_eq!(graph.to_string(), should_be.to_string());
}

#[allow(dead_code)]
fn build_graph() -> DirectedGraph<DummyLabel, DummyLabel> {
    let nodes = [A, B, A];
    let edges = [(0, 1, A), (2, 0, A), (2, 1, A)];
    DirectedGraph::from_vec(&nodes, &edges)
}

#[allow(dead_code)]
fn build_rule() -> Rule<DummyLabel, DummyLabel, DummyLabel, DummyLabel> {
    let s = DirectedGraph::from_vec(&[A, B], &[(0, 1, A)]);
    let r = DirectedGraph::from_vec(&[A, C], &[(0, 1, B), (1, 0, B)]);
    let mut h = HashMap::new();
    h.insert(0, 0);
    Rule::new(s, r, h)
}
