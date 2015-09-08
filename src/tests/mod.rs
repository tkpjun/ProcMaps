mod graph;
mod rules;
mod parsing;

use graph_grammar::labels::Symbol;
use graph_grammar::labels::RichSymbol;
use graph_grammar::labels::SymbolSet;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DummyLabel {A, B, C}
impl Symbol for DummyLabel { }
impl RichSymbol for DummyLabel { type Inner = (); }
impl SymbolSet<DummyLabel> for DummyLabel {
    fn is_superset_of(&self, other: &DummyLabel) -> bool {
        self == other
    }
}
