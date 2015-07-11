use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;

#[derive(Clone, Eq, PartialEq)]
pub enum NodeLabel<T: Eq + Clone> {
    Null,
    LevelEntry(T),
    LevelExit(T),
    AreaEntry(T),
    AreaExit(T),
    Chain(T),
    Fork(T),
    LevelBoss(T),
    MiniBoss(T),
    Test(T),
    Encounter(T),
    Puzzle(T),
    Key(T),
    Lock(T),
    MultiKey(T),
    MultiLock(T),
    Event(T),
    EventLock(T),
    SecretDoor(T),
    Item(T),
    SpecialItem(T),
    PointOfInterest(T),
    EmptyScenery(T),
}
impl<T: Eq + Clone> Symbol for NodeLabel<T> {}
impl<T: Eq + Clone> SymbolSet<NodeLabel<T>> for NodeLabel<T> {
    fn is_superset_of(&self, other: &NodeLabel<T>) -> bool {
        *self == *other
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum EdgeLabel<T: Eq + Clone> {
    TightCoup(NodeLabel<T>),
    LooseCoup(NodeLabel<T>),
}
impl<T: Eq + Clone> Symbol for EdgeLabel<T> {}
impl<T: Eq + Clone> SymbolSet<EdgeLabel<T>> for EdgeLabel<T> {
    fn is_superset_of(&self, other: &EdgeLabel<T>) -> bool {
        *self == *other
    }
}
