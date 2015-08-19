use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
use graph_grammar::graph::DirectedGraph;
use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeLabel {
    Room(Vec<MissionNode>),
    Corridor(MissionNode),
    Intersection(MissionNode),
    OpenTerrain(Vec<MissionNode>),
    Undeveloped(DirectedGraph<MissionNode, MissionEdge>, usize),
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
    LateralMove,
    Warp,
    //OneWayWarp,
    OpenTerrain,
}
impl Symbol for EdgeLabel {}
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}
