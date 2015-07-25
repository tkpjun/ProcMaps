mod graph;
mod rules;
use graph_grammar::labels::Symbol;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DummyLabel {A, B, C}
impl Symbol for DummyLabel {}
