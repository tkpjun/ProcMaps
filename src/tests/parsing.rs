use deserialization::json;
use graph_grammar::graph::DirectedGraph;
use mission_grammar::labels::NodeLabel::*;

#[test]
fn parse_ruleset() {
    let value = json::read_value("src/tests/ruleset.json").unwrap();
    json::mission_rules(&value).unwrap();
}

#[test]
fn use_ruleset() {
    let value = json::read_value("src/tests/ruleset.json").unwrap();
    let mut rules = json::mission_rules(&value).unwrap();
    rules.put_graph(DirectedGraph::from_vec(&[Null], &[]));
    assert!(rules.apply_rule(0));
    assert!(rules.apply_rule(1));
    assert!(rules.apply_rule(3));
    assert!(rules.apply_rule(3));
    //let graph = rules.take_graph();
    //println!("{}", graph.unwrap().to_string());
    //assert_eq!(" ", "");
}
