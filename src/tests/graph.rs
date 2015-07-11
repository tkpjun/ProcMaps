use graph_grammar::graph::DirectedGraph;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
enum Dummy {A, B, C}

#[test]
fn build_graph() {
    let graph = build();
    assert_eq!(graph.to_string(),
        "1,2,-> A -> 1,\n\
        0,-> B -> 0,2,\n\
        1,-> C -> 0,\n");
}
#[test]
fn remove_node() {
    let mut graph = build();
    graph.remove_node(0);
    assert_eq!(graph.to_string(),
        "1,-> C -> \n\
        -> B -> 0,\n");
}

#[allow(dead_code)]
fn build() -> DirectedGraph<Dummy, Dummy> {
    let mut g = DirectedGraph::new();
    g.push_node(Dummy::A);
    g.push_node(Dummy::B);
    g.push_node(Dummy::C);
    g.add_edge(0, 1, Dummy::A, false);
    g.add_edge(1, 2, Dummy::A, true);
    g.add_edge(2, 0, Dummy::A, true);
    return g;
}
