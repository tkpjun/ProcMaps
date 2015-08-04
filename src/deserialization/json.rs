use super::runtime_labels::NodeLabel as UntypedNode;
use super::runtime_labels::EdgeLabel as UntypedEdge;
use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;
use super::ser_symbol::SerSymbol;
use graph_grammar::graph::DirectedGraph;
use graph_grammar::rule::Rule;
use graph_grammar::labels::SearchLabel;
use graph_grammar::labels::SymbolSet;
use serde::json::{self, Value};
use serde::json::Error as jsonError;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn read_value(path: &str) -> Result<Value, jsonError> {
    read_file(&path).and_then(|file| json::from_str(&file))
}

pub fn mission_rules_simple(value: &Value) -> Result<Vec<Rule<MissionNode, MissionEdge, MissionNode, MissionEdge>>, jsonError> {
    none_checked(get_rules::<MissionNode, MissionEdge, MissionNode, MissionEdge>(value))
}

pub fn mission_rules(value: &Value) -> Result<Vec<Rule<MissionNode, MissionEdge, SearchLabel<MissionNode>, SearchLabel<MissionEdge>>>, jsonError> {
    none_checked(get_rules::<MissionNode, MissionEdge, SearchLabel<MissionNode>, SearchLabel<MissionEdge>>(value))
}

fn none_checked<T>(option: Option<Vec<Option<T>>>) -> Result<Vec<T>, jsonError> {
    let vec = match option {
        Some(v) => v,
        None => { return Err(jsonError::MissingFieldError("JSON object has an invalid top-level field!")); }
    };
    let mut ret = Vec::with_capacity(vec.capacity());
    for value in vec.into_iter() {
        match value {
            Some(thing) => { ret.push(thing); }
            None => { return Err(jsonError::MissingFieldError("Some rule object has an invalid field!")); }
        }
    }
    Ok(ret)
}

fn read_file(path: &str) -> Result<String, jsonError> {
    let path = Path::new(path);
    let mut content = String::new();
    try!(File::open(&path).and_then(|mut f| f.read_to_string(&mut content)));
    return Ok(content);
}

fn get_labels(value: &Value) -> (Vec<UntypedNode>, Vec<UntypedEdge>) {
    unimplemented!();
}

fn get_rules<S, T, U, V>(value: &Value) -> Option<Vec<Option<Rule<S, T, U, V>>>>
where S: SerSymbol, T: SerSymbol, U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    /*let obj = value.as_object();
    let label_set = match obj.and_then(|o| o.get("labelSet"))
                                           .and_then(Value::as_string) {
        Some("missionGrammar") => MissionGrammar,
        Some("other") => Other,
        None => Other,
        _ => return None,
    };
    let uses_search_labels = match obj.and_then(|o| o.get("searchLabels"))
                                      .and_then(Value::as_boolean) {
        Some(true) => true,
        _ => false,
    };*/
    value.as_object()
         .and_then(|o| o.get("rules"))
         .and_then(Value::as_array)
         .map(|vec| vec.into_iter()
                       .map(Value::as_object)
                       .map(|rule| create_rule::<S, T, U, V>(rule))
                       .collect()
         )
}

fn create_rule<S, T, U, V>(map: Option<&BTreeMap<String, Value>>) -> Option<Rule<S, T, U, V>>
where S: SerSymbol, T: SerSymbol, U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    let start = map.and_then(|m| m.get("start"))
                   .and_then(Value::as_object)
                   .and_then(|o| parse_start::<S, T, U, V>(o));
    let result = map.and_then(|m| m.get("result"))
                    .and_then(Value::as_object)
                    .and_then(|o| parse_result::<S, T>(o));
    let same = map.and_then(|m| m.get("sameNodes"))
                  .and_then(Value::as_array)
                  .and_then(parse_same_nodes);
    if start.as_ref().and(result.as_ref()).and(same.as_ref()).is_some() {
        Some(Rule::new(start.unwrap(), result.unwrap(), same.unwrap()))
    }
    else {
        None
    }
}

fn parse_start<S, T, U, V>(map: &BTreeMap<String, Value>) -> Option<DirectedGraph<U, V>>
where U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    let nodes = match map.get("nodes").and_then(Value::as_array) {
        Some(ns) => {
            let mut ret = Vec::new();
            for n in ns {
                if let Some(arr) = n.as_array() {
                    ret.reserve_exact(arr.len());
                    match arr.get(0).and_then(Value::as_string) {
                        Some(s) => {
                            if let Some(node) = U::parse(s, &n) {
                                ret.push(node);
                            }
                            else { return None; }
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
                    ret.reserve_exact(arr.len());
                    match (arr.get(0).and_then(Value::as_u64),
                           arr.get(1).and_then(Value::as_u64),
                           arr.get(2).and_then(Value::as_string)) {
                        (Some(begin), Some(end), Some(s)) => {
                            if let Some(edge) = V::parse(s, &e) {
                                ret.push((begin as usize, end as usize, edge));
                            }
                            else { return None; }
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

fn parse_result<S, T>(map: &BTreeMap<String, Value>) -> Option<DirectedGraph<S, T>>
where S: SerSymbol, T: SerSymbol {
    let nodes = match map.get("nodes").and_then(Value::as_array) {
        Some(ns) => {
            let mut ret = Vec::new();
            for n in ns {
                if let Some(arr) = n.as_array() {
                    ret.reserve_exact(arr.len());
                    match (arr.get(0).and_then(Value::as_string),
                           arr.get(1)) {
                        (Some(s), Some(i)) => {
                             if let Some(node) = S::parse(s, &i) {
                                 ret.push(node);
                             }
                             else { return None; }
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
                    ret.reserve_exact(arr.len());
                    match (arr.get(0).and_then(Value::as_u64),
                           arr.get(1).and_then(Value::as_u64),
                           arr.get(2).and_then(Value::as_string)) {
                        (Some(begin), Some(end), Some(s)) => {;
                            if let Some(edge) = T::parse(s, &Value::Null) {
                                ret.push((begin as usize, end as usize, edge));
                            }
                            else { return None; }
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
            match (v[0].as_u64(), v[1].as_u64() ) {
                (Some(i0), Some(i1)) => {ret.insert(i0 as usize, i1 as usize);},
                _ => { return None; }
            }
        }
        else { return None; }
    }
    Some(ret)
}
