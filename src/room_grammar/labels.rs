use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
use graph_grammar::graph::DirectedGraph;
use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeLabel {
    Room(MissionNode, u32),
    Corridor(MissionNode),
    Intersection(MissionNode, u32),
    Undeveloped(DirectedGraph<MissionNode, MissionEdge>, usize, u32),
}
impl Symbol for NodeLabel {}
impl SymbolSet<NodeLabel> for NodeLabel {
    fn is_superset_of(&self, other: &NodeLabel) -> bool {
        *self == *other
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EdgeLabel {
    Doorway,
    //OneWayDoorway,
    SecretDoor,
    ChangeZ(u32, u32),
    //OneWayElevChange(u32, u32),
    Warp,
    //OneWayWarp,
}
impl Symbol for EdgeLabel {}
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}
