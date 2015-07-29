use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;

pub fn mission_node(string: &str, value: i32) -> Option<MissionNode> {
    match string {
        "LevelEntry" => Some(MissionNode::LevelEntry(value)),
        "LevelExit" => Some(MissionNode::LevelExit(value)),
        _ => None
    }
}

pub fn mission_edge(string: &str) -> Option<MissionEdge> {
    match string {
        "Tight" => Some(MissionEdge::Tight),
        "Loose" => Some(MissionEdge::Loose),
        _ => None
    }
}
