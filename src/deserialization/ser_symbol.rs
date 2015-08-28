use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;
use mission_grammar::labels::{AbilityId, KeyId};
use graph_grammar::labels::Symbol;
use graph_grammar::labels::SearchLabel;
use serde::json::Value;

pub trait SerSymbol: Symbol {
    fn parse(&str, &Value) -> Option<Self>;
}

impl SerSymbol for MissionNode {
    fn parse(name: &str, value: &Value) -> Option<MissionNode> {
        //also take string keywords such as "uniq" for dynamic values?
        //and random range arrays?
        if let Some(i) = value.as_i64() {
            let sub = i as i32;
            match name {
                "AreaEntry" => Some(MissionNode::AreaEntry(sub)),
                "AreaExit" => Some(MissionNode::AreaExit(sub)),
                "Pattern" => Some(MissionNode::Pattern(sub)),
                "LinearChain" => Some(MissionNode::LinearChain(AbilityId(sub))),
                "ParallelChain" => Some(MissionNode::ParallelChain(AbilityId(sub))),
                "Fork" => Some(MissionNode::Fork(AbilityId(sub))),
                "DeadEnd" => Some(MissionNode::DeadEnd(AbilityId(sub))),
                "Gate" => Some(MissionNode::Gate(AbilityId(sub))),
                "Challenge" => Some(MissionNode::Challenge(AbilityId(sub))),
                "Enemies" => Some(MissionNode::Enemies(AbilityId(sub))),
                "Puzzle" => Some(MissionNode::Puzzle(AbilityId(sub))),
                "Boss" => Some(MissionNode::Boss(sub)),
                "MiniBoss" => Some(MissionNode::MiniBoss(sub)),
                "Friendly" => Some(MissionNode::Friendly(sub)),
                "Key" => Some(MissionNode::Key(KeyId(sub))),
                "Lock" => Some(MissionNode::Lock(KeyId(sub), 1)),
                "Trigger" => Some(MissionNode::Trigger(KeyId(sub))),
                "TriggerLock" => Some(MissionNode::TriggerLock(KeyId(sub), 1)),
                "Secret" => Some(MissionNode::Secret(sub)),
                "Reward" => Some(MissionNode::Reward(sub)),
                "AbilityItem" => Some(MissionNode::AbilityItem(AbilityId(sub))),
                "Tutorial" => Some(MissionNode::Tutorial(AbilityId(sub))),
                "Scenery" => Some(MissionNode::Scenery(sub)),
                _ => None
            }
        }
        else {
            match name {
                "Null" => Some(MissionNode::Null),
                "LevelEntry" => Some(MissionNode::LevelEntry),
                "LevelExit" => Some(MissionNode::LevelExit),
                _ => None
            }
        }
    }
}

impl SerSymbol for SearchLabel<MissionNode> {
    fn parse(name: &str, value: &Value) -> Option<SearchLabel<MissionNode>> {
        let arr = match value.as_array() {
            Some(a) => a,
            None => { return None; }
        };
        let mut content = Vec::with_capacity(arr.len() - 1);
        for opt in arr.into_iter().skip(1).map(Value::as_array) {
            match opt {
                Some(a) => match a[0].as_string() {
                        Some(s)=> content.push((s, a.get(1).unwrap_or(&Value::Null).clone())),
                        _ => { return None; }
                    },
                None => { return None; }
            }
        }
        match name {
            "Is" => match MissionNode::parse(&content[0].0, &content[0].1) {
                Some(node) => Some(SearchLabel::Is(node)),
                None => None
            },
            "Some" => {
                let mut vec = Vec::with_capacity(content.len());
                for piece in content {
                    match MissionNode::parse(&piece.0, &piece.1) {
                        Some(node) => vec.push(node),
                        None => { return None; }
                    }
                }
                Some(SearchLabel::Some(vec))
            },
            "Not" => {
                let mut vec = Vec::with_capacity(content.len());
                for piece in content {
                    match MissionNode::parse(&piece.0, &piece.1) {
                        Some(node) => vec.push(node),
                        None => { return None; }
                    }
                }
                Some(SearchLabel::Not(vec))
            },
            "Any" => Some(SearchLabel::Any),
            _ =>  None
        }
    }
}

impl SerSymbol for MissionEdge {
    fn parse(name: &str, _: &Value) -> Option<MissionEdge> {
        match name {
            "Tight" => Some(MissionEdge::Tight),
            "Loose" => Some(MissionEdge::Loose),
            _ => None
        }
    }
}

impl SerSymbol for SearchLabel<MissionEdge> {
    fn parse(name: &str, value: &Value) -> Option<SearchLabel<MissionEdge>> {
        let arr = match value.as_array() {
            Some(a) => a,
            None => { return None; }
        };
        let mut content = Vec::with_capacity(arr.len() - 1);
        for opt in arr.into_iter().skip(3).map(Value::as_string) {
            match opt {
                Some(s) => content.push(s),
                None => { return None; }
            }
        }
        let null = Value::Null;
        match name {
            "Is" => match MissionEdge::parse(&content[0], &null) {
                Some(edge) => Some(SearchLabel::Is(edge)),
                None => None
            },
            "Some" => {
                let mut vec = Vec::with_capacity(content.len());
                for piece in content {
                    match MissionEdge::parse(&piece, &null) {
                        Some(edge) => vec.push(edge),
                        None => { return None; }
                    }
                }
                Some(SearchLabel::Some(vec))
            },
            "Not" => {
                let mut vec = Vec::with_capacity(content.len());
                for piece in content {
                    match MissionEdge::parse(&piece, &null) {
                        Some(edge) => vec.push(edge),
                        None => { return None; }
                    }
                }
                Some(SearchLabel::Not(vec))
            },
            "Any" => Some(SearchLabel::Any),
            _ =>  None
        }
    }
}
