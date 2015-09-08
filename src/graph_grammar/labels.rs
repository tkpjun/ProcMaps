use std::fmt::Debug;

pub trait InnerData: Clone + PartialEq {
    fn is_special_var(&self) -> Option<i32>;

    fn get_special_var(i32) -> Option<Self>;

    fn increment(&mut self, amount: i32);

    fn is_superset_of(&self, other: &Self) -> bool {
        if self.is_special_var().is_some() && other.is_special_var().is_none() {
            true
        }
        else {
            self == other
        }
    }
}
impl InnerData for () {
    fn is_special_var(&self) -> Option<i32> {
        None
    }
    fn get_special_var(_: i32) -> Option<()> {
        None
    }
    fn increment(&mut self, _: i32) { }
}

pub trait RichSymbol: Symbol {
    type Inner: InnerData;

    fn has_inner(&self) -> bool {
        false
    }
    fn get_inner(&self) -> Option<Self::Inner> {
        None
    }
    fn set_inner(&mut self, _: &Self::Inner) {
    }
}

pub trait Symbol: PartialEq + Clone + Debug {}

pub trait SymbolSet<T: Symbol>: Symbol {
    fn is_superset_of(&self, other: &T) -> bool;
}
/*
#[derive(Clone, PartialEq, Debug)]
pub enum SearchLabel<T: Symbol + SymbolSet<T>> {
    Some(Vec<T>),
    Not(Vec<T>),
    Is(T),
    Any,
}
impl<T: Symbol + SymbolSet<T>> Symbol for SearchLabel<T> { }
impl<T: Symbol + SymbolSet<T>> SymbolSet<T> for SearchLabel<T> {
    fn is_superset_of(&self, other: &T) -> bool {
        match self {
            &SearchLabel::Any => true,
            &SearchLabel::Is(ref label) => label.is_superset_of(other),
            &SearchLabel::Some(ref vec) => vec.iter().any(|label| label.is_superset_of(other)),
            &SearchLabel::Not(ref vec) => vec.iter().all(|label| !label.is_superset_of(other))
        }
    }
}*/
