pub mod graph;
pub mod rule;
pub mod labels;
pub mod contract;
pub mod ruleset;

use std::borrow::Borrow;

pub enum Either<S, T: Borrow<[S]>> { Single(S), List(T) }
