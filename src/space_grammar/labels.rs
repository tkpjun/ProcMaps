use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
use graph_grammar::graph::DirectedGraph;
use room_grammar::labels::NodeLabel as RoomNode;
use room_grammar::labels::EdgeLabel as RoomEdge;

pub struct Space { x: u32, y: u32, w: u32, h: u32 }

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeLabel {
    Rect(Space, u32, RoomNode),
    RectSet(Vec<Space>, u32, RoomNode),
    FreeForm(Vec<(u32, u32)>, u32, RoomNode),
    Undeveloped(Vec<Space>, DirectedGraph<RoomNode, RoomEdge>, usize),
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
