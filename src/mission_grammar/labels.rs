use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
//use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeLabel {
    Null,
    LevelEntry,
    LevelExit,
    UndevelopedArea(i32),
    AreaEntry(i32),
    AreaExit(i32),
    Chain(i32),
    Fork(i32),
    Boss(i32),
    MiniBoss(i32),
    Test(i32),
    Trap(i32),
    Encounter(i32),
    FriendlyEncounter(i32),
    Puzzle(i32),
    Key(i32),
    Lock(i32),
    MultiLock(Vec<i32>),
    Event(i32),
    EventLock(i32),
    MultiEventLock(Vec<i32>),
    Secret(i32),
    Item(i32),
    SpecialItem(i32),
    Reward(i32),
    Scenery(i32),
}
impl Symbol for NodeLabel {}
impl SymbolSet<NodeLabel> for NodeLabel {
    fn is_superset_of(&self, other: &NodeLabel) -> bool {
        *self == *other
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EdgeLabel {
    Tight,
    Loose,
}
impl Symbol for EdgeLabel {}
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}

/*#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeLabel<T: Eq + Clone + Debug> {
    Init,
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
    Reward(T),
    Scenery(T),
}
impl<T: Eq + Clone + Debug> Symbol for NodeLabel<T> {}
impl<T: Eq + Clone + Debug> SymbolSet<NodeLabel<T>> for NodeLabel<T> {
    fn is_superset_of(&self, other: &NodeLabel<T>) -> bool {
        *self == *other
    }
}*/
