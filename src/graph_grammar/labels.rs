use std::fmt::Debug;

pub trait Symbol: Eq + Clone + Debug {}

pub trait SymbolSet<T: Symbol>: Symbol {
    fn is_superset_of(&self, other: &T) -> bool;
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SearchNode<T: Symbol> {
    Some(Vec<T>),
    Not(Vec<T>),
    Any,
}
impl<T: Symbol> Symbol for SearchNode<T> {}
impl<T: Symbol> SymbolSet<T> for SearchNode<T> {
    fn is_superset_of(&self, other: &T) -> bool {
        match self {
            &SearchNode::Any => true,
            &SearchNode::Some(ref a) => a.iter().any(|label| other == label),
            //technically wrong, not(a) is the superset
            &SearchNode::Not(ref a) => a.iter().all(|label| other != label)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SearchEdge<T: Symbol> {
    Some(Vec<T>),
    Not(Vec<T>),
    Any,
}
impl<T: Symbol> Symbol for SearchEdge<T> {}
impl<T: Symbol> SymbolSet<T> for SearchEdge<T> {
    fn is_superset_of(&self, other: &T) -> bool {
        match self {
            &SearchEdge::Any => true,
            &SearchEdge::Some(ref a) => a.iter().any(|label| other == label),
            &SearchEdge::Not(ref a) => a.iter().all(|label| other != label)
        }
    }
}
