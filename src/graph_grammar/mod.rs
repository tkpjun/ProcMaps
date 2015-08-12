pub mod graph;
pub mod rule;
pub mod labels;
pub mod contract;
pub mod ruleset;

pub enum Either<S, T> { One(S), Another(T) }
