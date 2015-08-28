use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AbilityId(pub i32);
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct KeyId(pub i32);

#[derive(Clone, PartialEq, Debug)]
pub enum NodeLabel {
    Null,
    LevelEntry,
    LevelExit,
    UndevelopedArea(i32),
    AreaEntry(i32),
    AreaExit(i32),
    Pattern(i32),
    LinearChain(AbilityId),//may require the corresponding ability or any before it
    ParallelChain(AbilityId),//may require the corresponding ability or any before it
    Fork(AbilityId),//may require the corresponding ability or any before it,
    DeadEnd(AbilityId),//may require the corresponding ability or any before it
    Gate(AbilityId),//requires the corresponding ability, may require any before it
    Boss(i32),
    MiniBoss(i32),
    Challenge(AbilityId),//requires the corresponding ability, may require any before it
    Enemies(AbilityId),//requires the corresponding ability, may require any before it
    Puzzle(AbilityId),//requires the corresponding ability, may require any before it
    Key(KeyId),
    Lock(KeyId, u32),
    Trigger(KeyId),
    TriggerLock(KeyId, u32),
    Secret(i32),
    Reward(i32),
    AbilityItem(AbilityId),
    Tutorial(AbilityId),//for the corresponding ability
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
