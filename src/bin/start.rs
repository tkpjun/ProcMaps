extern crate proc_maps;

pub use proc_maps::mission_graph::graph;
pub use proc_maps::mission_graph::grammar;
pub use proc_maps::mission_graph::grammar::Symbol;
pub use proc_maps::mission_graph::grammar::PathType;

fn main() {
    let mut graph = graph::Graph::new();
    graph.push_node(Symbol::LevelEntr);
    graph.push_node(Symbol::LevelExit);
    graph.add_path(0, 1);
    println!("Before:\n{}", graph.to_string());

    let s = vec![Symbol::LevelEntr, Symbol::LevelExit];
    let mut e = graph::Graph::new();
    e.push_node(Symbol::LevelEntr);
    e.push_node(Symbol::Key(0));
    e.push_node(Symbol::KeyDoor(vec![0]));
    e.push_node(Symbol::LevelExit);
    e.add_path(0, 1);
    e.add_path(1, 2);
    e.add_path(2, 3);
    let p = vec![PathType::Tight];
    let rule = grammar::Rule{ start:s, s_paths:p, result:e, anchor:((0, 0),(1, 3))};

    //graph.apply_rule(rule);
    graph.apply_rule(&rule);
    println!("After:\n{}", graph.to_string());
}
