use graph_grammar::labels::Symbol;
use graph_grammar::labels::RichSymbol;
use graph_grammar::labels::SymbolSet;
use graph_grammar::labels::InnerData;
use std::i32;
use self::NodeLabel::*;

const X: i32 = i32::MIN + 9;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AbilityId(pub i32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct KeyId(pub i32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Id(pub i32);

impl InnerData for Id {
    fn is_special_var(&self) -> Option<i32> {
        let &Id(i) = self;
        if i <= X + 18 { Some(i - X) } else { None }
    }
    fn get_special_var(id: i32) -> Option<Id> {
        match id {
            a @ -9...9 => Some(Id(a + X)),
            _ => None
        }
    }
    fn increment(&mut self, amount: i32) {
        let &mut Id(i) = self;
        *self = Id(i + amount);
    }
}

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

impl RichSymbol for NodeLabel {
    type Inner = Id;

    fn has_inner(&self) -> bool {
        match self {
            &LinearChain(AbilityId(_)) |
            &ParallelChain(AbilityId(_)) |
            &Fork(AbilityId(_)) |
            &DeadEnd(AbilityId(_)) |
            &Gate(AbilityId(_)) |
            &Enemies(AbilityId(_)) |
            &Challenge(AbilityId(_)) |
            &Puzzle(AbilityId(_)) |
            &AbilityItem(AbilityId(_)) |
            &Tutorial(AbilityId(_))
            => true,
            _ => false
        }
    }
    fn get_inner(&self) -> Option<Self::Inner> {
        match self {
            &LinearChain(AbilityId(a)) |
            &ParallelChain(AbilityId(a)) |
            &Fork(AbilityId(a)) |
            &DeadEnd(AbilityId(a)) |
            &Gate(AbilityId(a)) |
            &Enemies(AbilityId(a)) |
            &Challenge(AbilityId(a)) |
            &Puzzle(AbilityId(a)) |
            &AbilityItem(AbilityId(a)) |
            &Tutorial(AbilityId(a))
            => Some(Id(a)),
            _ => None
        }
    }
    fn set_inner(&mut self, new: &Id) {
        let Id(a) = new.clone();
        match self {
            &mut LinearChain(_) => *self = LinearChain(AbilityId(a)),
            &mut ParallelChain(_) => *self = ParallelChain(AbilityId(a)),
            &mut Fork(_) => *self = Fork(AbilityId(a)),
            &mut DeadEnd(_) => *self = DeadEnd(AbilityId(a)),
            &mut Gate(_) => *self = Gate(AbilityId(a)),
            &mut Challenge(_) => *self = Challenge(AbilityId(a)),
            &mut Enemies(_) => *self = Enemies(AbilityId(a)),
            &mut Puzzle(_) => *self = Puzzle(AbilityId(a)),
            &mut AbilityItem(_) => *self = AbilityItem(AbilityId(a)),
            &mut Tutorial(_) => *self = Tutorial(AbilityId(a)),
            _ => {}
        }
    }
}
impl SymbolSet<NodeLabel> for NodeLabel {
    fn is_superset_of(&self, other: &NodeLabel) -> bool {
        if let Some(_) = self.get_inner().and_then(|a| a.is_special_var()) {
            if let Some(_) = other.get_inner().and_then(|b| b.is_special_var()) {
                *self == *other
            }
            else { true }
        }
        else { *self == *other }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EdgeLabel {
    Tight,
    Loose,
}
impl Symbol for EdgeLabel { }
impl SymbolSet<EdgeLabel> for EdgeLabel {
    fn is_superset_of(&self, other: &EdgeLabel) -> bool {
        *self == *other
    }
}
