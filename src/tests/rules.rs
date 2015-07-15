use graph_grammar::graph::DirectedGraph;
use graph_grammar::rule::Rule;
use graph_grammar::labels::SearchNode;
use graph_grammar::labels::SearchEdge;
use std::collections::HashMap;
use tests::graph::DummyLabel;
use tests::graph::DummyLabel::{A, B, C};

#[test]
fn find_subgraphs() {
    let mut graph = build_graph();
    let rule = build_rule();
    let subgraphs = rule.find_subgraphs(&graph);
    assert!(subgraphs.len() == 2);
    assert_eq!(format!("{:?}", subgraphs[0]), "[0, 1]");
}

#[test]
fn apply_rule() {
    let mut graph = build_graph();
    let rule = build_rule();
    let subgraphs = rule.find_subgraphs(&graph);
    rule.apply_to(&mut graph, &subgraphs[0]);
    assert_eq!(graph.to_string(),
    "1,2,-> A -> B(0-2),\n\
    -> A -> A(1-0),\n\
    0,-> C -> B(2-0),\n");
}

#[allow(dead_code)]
fn build_graph() -> DirectedGraph<DummyLabel, DummyLabel> {
    let mut g = DirectedGraph::new();
    g.push_node(A);
    g.push_node(B);
    g.push_node(A);
    g.add_edge(0, 1, A, true);
    g.add_edge(2, 0, A, true);
    g.add_edge(2, 1, A, true);
    return g;
}

#[allow(dead_code)]
fn build_rule() -> Rule<DummyLabel, DummyLabel, SearchNode<DummyLabel>, SearchEdge<DummyLabel>> {
    let mut s = DirectedGraph::new();
    s.push_node(SearchNode::Some(vec!(A)));
    s.push_node(SearchNode::Some(vec!(B)));
    s.add_edge(0, 1, SearchEdge::Any, true);

    let mut r = DirectedGraph::new();
    r.push_node(A);
    r.push_node(C);
    r.add_edge(0, 1, B, false);

    let mut h = HashMap::new();
    h.insert(0, 0);

    Rule::new(s, r, h, 0)
}
