extern crate proc_maps;

pub use proc_maps::graph_grammar::graph;
pub use proc_maps::mission_graph::grammar;
pub use proc_maps::mission_graph::grammar::Symbol;
pub use proc_maps::mission_graph::grammar::PathType;
pub use proc_maps::mission_graph::grammar::Anchor;

fn main() {
    let mut graph = graph::Graph::new();
    //graph.push_node(Symbol::LevelEntr);
    graph.push_node(Symbol::Combat(0));
    //graph.push_node(Symbol::LevelExit);
    //graph.add_path(0, 1);
    //graph.add_path(1, 2);
    println!("Before:\n{}", graph.to_string());

    let s = vec![Symbol::Combat(0)];
    let p = vec![PathType::Loose];
    let mut e = graph::Graph::new();
    e.push_node(Symbol::Door);
    e.push_node(Symbol::Combat(0));
    e.push_node(Symbol::Loot(0));
    e.add_path(0, 1);
    e.add_path(1, 2);
    let rule = grammar::Rule{ start:s, s_paths:p, result:e, anchor:((0, 1), None)};

    graph.apply_rule(&rule);
    println!("After:\n{}", graph.to_string());
}
