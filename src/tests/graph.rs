use graph_grammar::graph::DirectedGraph;
use graph_grammar::labels::Symbol;
use super::DummyLabel::{A, B, C};

#[test]
fn build_graph() {
    let graph = build();
    assert_eq!(graph.to_string(),
        "1,2,-> A -> A(0-1),\n\
        0,-> B -> A(1-0),A(1-2),\n\
        1,-> C -> A(2-0),\n");
}
#[test]
fn remove_node() {
    let mut graph = build();
    graph.remove_node(0);
    assert_eq!(graph.to_string(),
        "1,-> C -> \n\
        -> B -> A(1-0),\n");
}

#[allow(dead_code)]
fn build() -> DirectedGraph<DummyLabel, DummyLabel> {
    let mut g = DirectedGraph::new();
    g.push_node(A);
    g.push_node(B);
    g.push_node(C);
    g.add_edge(0, 1, A, false);
    g.add_edge(1, 2, A, true);
    g.add_edge(2, 0, A, true);
    return g;
}
