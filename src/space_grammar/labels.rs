use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
use graph_grammar::graph::DirectedGraph;
use room_grammar::labels::NodeLabel as RoomNode;
use room_grammar::labels::EdgeLabel as RoomEdge;

pub struct Location { x: i32, y: i32 }

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeLabel {
    Rect(Location, i32, RoomNode),
    RectSet(Vec<Location>, i32, RoomNode),
    FreeForm(Vec<Location>, i32, RoomNode),
    Undeveloped(Vec<Location>, DirectedGraph<RoomNode, RoomEdge>, usize),
}
impl Symbol for NodeLabel {}
impl SymbolSet<NodeLabel> for NodeLabel {
    fn is_superset_of(&self, other: &NodeLabel) -> bool {
        *self == *other
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EdgeLabel {
    DirectConn(RoomEdge),
    IndirectConn(RoomEdge),
    OneWayDirectConn(RoomEdge),
    OneWayIndirectConn(RoomEdge),
}
impl Symbol for EdgeLabel {}
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}
