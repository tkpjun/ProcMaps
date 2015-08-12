use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
use super::ser_symbol::SerSymbol;
use graph_grammar::labels::SearchLabel;
use serde::json::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct NodeLabel {
    pub name: String,
    pub data: Value,
}
impl Symbol for NodeLabel {}
impl SymbolSet<NodeLabel> for NodeLabel {
    fn is_superset_of(&self, other: &NodeLabel) -> bool {
        *self == *other
    }
}

impl SerSymbol for NodeLabel {
    fn parse(name: &str, value: &Value) -> Option<NodeLabel> {
        unimplemented!()
    }
}
impl SerSymbol for SearchLabel<NodeLabel> {
    fn parse(name: &str, value: &Value) -> Option<SearchLabel<NodeLabel>> {
        unimplemented!()
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EdgeLabel {
    pub name: String,
}
impl Symbol for EdgeLabel {}
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}

impl SerSymbol for EdgeLabel {
    fn parse(name: &str, value: &Value) -> Option<EdgeLabel> {
        unimplemented!()
    }
}
impl SerSymbol for SearchLabel<EdgeLabel> {
    fn parse(name: &str, value: &Value) -> Option<SearchLabel<EdgeLabel>> {
        unimplemented!()
    }
}
