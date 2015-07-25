mod graph;
mod rules;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DummyLabel {A, B, C}
impl Symbol for DummyLabel {}
