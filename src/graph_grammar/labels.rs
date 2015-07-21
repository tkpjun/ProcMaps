use std::fmt::Debug;

pub trait Symbol: Eq + Clone + Debug {}

pub trait SymbolSet<T: Symbol>: Symbol {
    fn is_superset_of(&self, other: &T) -> bool;
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SearchLabel<T: Symbol> {
    Some(Vec<T>),
    Not(Vec<T>),
    Is(T),
    Any,
}
impl<T: Symbol> Symbol for SearchLabel<T> {}
impl<T: Symbol> SymbolSet<T> for SearchLabel<T> {
    fn is_superset_of(&self, other: &T) -> bool {
        match self {
            &SearchLabel::Any => true,
            &SearchLabel::Is(ref label) => label == other,
            &SearchLabel::Some(ref vec) => vec.iter().any(|label| other == label),
            &SearchLabel::Not(ref vec) => vec.iter().all(|label| other != label)
        }
    }
}
/*
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
}*/
