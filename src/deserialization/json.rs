use super::runtime_labels::NodeLabel as UntypedNode;
use super::runtime_labels::EdgeLabel as UntypedEdge;
use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;
use super::mappings::*;
use graph_grammar::graph::DirectedGraph;
use graph_grammar::rule::Rule;
use graph_grammar::labels::SearchLabel;
use graph_grammar::labels::Symbol;
use graph_grammar::labels::SymbolSet;
use serde::json::{self, Value};
use serde::json::Error as jsonError;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashMap;

enum LabelSet {
    Other,
    MissionGrammar,
}

pub fn mission_rules_simple(path: String) -> Result<Vec<Rule<MissionNode, MissionEdge, MissionNode, MissionEdge>>, jsonError> {
    let value = try!(read_file(&path).and_then(|file| json::from_str(&file)));
    none_checked(get_rules::<MissionNode, MissionEdge, MissionNode, MissionEdge>(&value))
}

pub fn mission_rules(path: String) -> Result<Vec<Rule<MissionNode, MissionEdge, SearchLabel<MissionNode>, SearchLabel<MissionEdge>>>, jsonError> {
    let value = try!(read_file(&path).and_then(|file| json::from_str(&file)));
    none_checked(get_rules::<MissionNode, MissionEdge, SearchLabel<MissionNode>, SearchLabel<MissionEdge>>(&value))
}

fn none_checked<T>(option: Option<Vec<Option<T>>>) -> Result<Vec<T>, jsonError> {
    unimplemented!();
}

fn read_file(path: &str) -> Result<String, jsonError> {
    let path = Path::new(path);
    let mut content = String::new();
    let mut file = try!(File::open(&path).and_then(|mut f| f.read_to_string(&mut content)));
    return Ok(content);
}

fn get_labels(value: &Value) -> (Vec<UntypedNode>, Vec<UntypedEdge>) {
    unimplemented!();
}

fn get_rules<S, T, U, V>(value: &Value) -> Option<Vec<Option<Rule<S, T, U, V>>>>
where S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T> {
    let obj = value.as_object();
    let label_set = match obj.and_then(|o| o.get("labelSet"))
                                           .and_then(Value::as_string) {
        Some("missionGrammar") => LabelSet::MissionGrammar,
        Some("other") => LabelSet::Other,
        None => LabelSet::Other,
        _ => return None,
    };
    let uses_search_labels = match obj.and_then(|o| o.get("searchLabels"))
                                      .and_then(Value::as_boolean) {
        Some(true) => true,
        _ => false,
    };
    value.as_object()
         .and_then(|o| o.get("rules"))
         .and_then(Value::as_array)
         .map(|vec| vec.into_iter()
                       .map(Value::as_object)
                       .map(|rule| create_rule::<S, T, U, V>(rule, &label_set, !uses_search_labels))
                       .collect()
         )
}

fn create_rule<S, T, U, V>(map: Option<&BTreeMap<String, Value>>, label_set: &LabelSet, is_simple: bool) -> Option<Rule<S, T, U, V>>
where S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T> {
    let start = map.and_then(|m| m.get("start"))
                    .and_then(Value::as_object)
                    .and_then(|o| parse_start::<S, T, U, V>(o, label_set, is_simple));
    let result = map.and_then(|m| m.get("result"))
                     .and_then(Value::as_object)
                     .and_then(|o| parse_result::<S, T>(o, label_set));
    let same = map.and_then(|m| m.get("same_nodes"))
                   .and_then(Value::as_array)
                   .and_then(parse_same_nodes);
    if start.as_ref().and(result.as_ref()).and(same.as_ref()).is_some() {
        Some(Rule::new(start.unwrap(), result.unwrap(), same.unwrap()))
    }
    else {
        None
    }
}

fn parse_start<S, T, U, V>(map: &BTreeMap<String, Value>, label_set: &LabelSet, is_simple: bool) -> Option<DirectedGraph<U, V>>
where S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T> {
    let nodes = match map.get("nodes").and_then(Value::as_array) {
        Some(ns) => {
            let mut ret = Vec::new();
            for n in ns {
                if let Some(arr) = n.as_array() {
                    if (is_simple) {
                        match (arr[0].as_string(), arr[1].as_i64()) {
                            (Some(s), Some(i)) => {
                                ret.push(parse_node::<U>(s, i as i32, label_set));
                            },
                            _ => { return None; }
                        }
                    }
                    else {
                        unimplemented!();
                    }
                }
                else { return None; }
            }
            ret
        },
        None => { return None; }
    };
    let edges = match map.get("edges").and_then(Value::as_array) {
        Some(es) => {
            let mut ret = Vec::new();
            for e in es {
                if let Some(arr) = e.as_array() {
                    if (is_simple) {
                        match (arr[0].as_string(), arr[1].as_u64(), arr[2].as_u64()) {
                            (Some(s), Some(begin), Some(end)) => {
                                let b = begin as usize;
                                let e = end as usize;
                                ret.push((parse_edge::<V>(s, b, e, label_set), b, e));
                            },
                            _ => { return None; }
                        }
                    }
                    else {
                        unimplemented!();
                    }
                }
                else { return None; }
            }
            ret
        },
        None => { return None; }
    };
    Some(DirectedGraph::from_vec(&nodes, &edges))
}

fn parse_result<S, T>(map: &BTreeMap<String, Value>, label_set: &LabelSet) -> Option<DirectedGraph<S, T>>
where S: Symbol, T: Symbol {
    let nodes = match map.get("nodes").and_then(Value::as_array) {
        Some(ns) => {
            let mut ret = Vec::new();
            for n in ns {
                if let Some(arr) = n.as_array() {
                    match (arr[0].as_string(), arr[1].as_i64()) {
                        (Some(s), Some(i)) => {
                            ret.push(parse_node::<S>(s, i as i32, label_set));
                        },
                        _ => { return None; }
                    }
                }
                else { return None; }
            }
            ret
        },
        None => { return None; }
    };
    let edges = match map.get("edges").and_then(Value::as_array) {
        Some(es) => {
            let mut ret = Vec::new();
            for e in es {
                if let Some(arr) = e.as_array() {
                    match (arr[0].as_string(), arr[1].as_u64(), arr[2].as_u64()) {
                        (Some(s), Some(begin), Some(end)) => {
                            let b = begin as usize;
                            let e = end as usize;
                            ret.push((parse_edge::<T>(s, b, e, label_set), b, e));
                        },
                        _ => { return None; }
                    }
                }
                else { return None; }
            }
            ret
        },
        None => { return None; }
    };
    Some(DirectedGraph::from_vec(&nodes, &edges))
}

fn parse_same_nodes(vec: &Vec<Value>) -> Option<HashMap<usize, usize>> {
    let mut ret = HashMap::new();
    for arr in vec.into_iter() {
        if let Some(v) = arr.as_array() {
            if let Some(i0) = vec[0].as_u64() {
                if let Some(i1) = vec[1].as_u64() {
                    ret.insert(i0 as usize, i1 as usize);
                }
                else { return None; }
            }
            else { return None; }
        }
        else { return None; }
    }
    Some(ret)
}

fn parse_node<T>(name: &str, value: i32, label_set: &LabelSet) -> T {
    unimplemented!();
}

fn parse_edge<T>(name: &str, from: usize, to: usize, label_set: &LabelSet) -> T {
    unimplemented!();
}
