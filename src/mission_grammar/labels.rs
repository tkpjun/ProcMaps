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
    Pattern(i32),
    LinearChain,
    ParallelChain,
    Fork,
    DeadEnd,
    Gate(i32),//requires the corresponding ability, may require any before it
    Boss(i32),//requires the corresponding ability, may require any before it
    MiniBoss(i32),
    Challenge(i32),//requires the corresponding ability, may require any before it
    Enemies(i32),//requires the corresponding ability, may require any before it
    Puzzle(i32),//requires the corresponding ability, may require any before it
    Key(i32),
    Lock(i32, u32),
    Trigger(i32),
    TriggerLock(i32, u32),
    Secret(i32),
    Reward(i32),
    AbilityItem(i32),
    Tutorial(i32),//for the corresponding ability
    Friendly(i32),
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
