extern crate proc_maps;

pub use proc_maps::grammar::graph;
pub use proc_maps::grammar::symbols;
pub use proc_maps::grammar::symbols::Symbol;

fn main() {
    let mut graph = graph::Graph::new();
    graph.push_node(Symbol::LevelEntr);
    graph.push_node(Symbol::LevelExit);
    graph.add_path(0, 1);
    println!("{}", graph.to_string());
    println!("");

    let mut s = Vec::new();
    s.push(Box::new(Symbol::LevelEntr));
    s.push(Box::new(Symbol::LevelExit));
    let mut e = Vec::new();
    e.push(Box::new(Symbol::LevelEntr));
    e.push(Box::new(Symbol::Key(0)));
    e.push(Box::new(Symbol::Lock(0)));
    e.push(Box::new(Symbol::LevelExit));
    let rule = symbols::Rule{ start:s, end:e, anchors:(0, 3)};

    graph.apply_rule(rule);
    println!("{}", graph.to_string());
}
