//use std::collections::HashMap;
use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeLabel {
    name: String,
    subtype: u32,
}
impl Symbol for NodeLabel {}
impl SymbolSet<NodeLabel> for NodeLabel {
    fn is_superset_of(&self, other: &NodeLabel) -> bool {
        *self == *other
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EdgeLabel {
    name: String,
    target: NodeLabel,
}
impl Symbol for EdgeLabel {}
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}
