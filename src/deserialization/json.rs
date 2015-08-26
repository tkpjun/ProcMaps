use super::runtime_labels::NodeLabel as UntypedNode;
use super::runtime_labels::EdgeLabel as UntypedEdge;
use mission_grammar::labels::NodeLabel as MissionNode;
use mission_grammar::labels::EdgeLabel as MissionEdge;
use super::ser_symbol::SerSymbol;
use graph_grammar::graph::DirectedGraph;
use graph_grammar::rule::Rule;
use graph_grammar::ruleset::RuleSet;
use graph_grammar::labels::SearchLabel;
use graph_grammar::labels::SymbolSet;
use graph_grammar::contract::GraphContract;
use graph_grammar::Either;
use serde::json::{self, Value};
use serde::json::Error as JsonError;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::default::Default;
use std::mem;

pub fn read_value(path: &str) -> Result<Value, JsonError> {
    read_file(&path).and_then(|file| json::from_str(&file))
}

pub fn mission_rules_simple(value: &Value) -> Result<RuleSet<MissionNode, MissionEdge, MissionNode, MissionEdge>, JsonError> {
    get_ruleset::<MissionNode, MissionEdge, MissionNode, MissionEdge>(value)
}

pub fn mission_rules(value: &Value) -> Result<RuleSet<MissionNode, MissionEdge, SearchLabel<MissionNode>, SearchLabel<MissionEdge>>, JsonError> {
    get_ruleset::<MissionNode, MissionEdge, SearchLabel<MissionNode>, SearchLabel<MissionEdge>>(value)
}

fn get_ruleset<S, T, U, V>(value: &Value) -> Result<RuleSet<S, T, U, V>, JsonError>
where S: SerSymbol, T: SerSymbol, U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    let rules_and_weights = try!(err_checked(get_rules(value)));
    let contract = try!(get_contract(value));
    let function = |weights: &[f32], index: usize, _: i32| -> f32 {
        weights[index] / 2.0
    };
    let weights = rules_and_weights.iter().map(|&(_, w)| w).collect();
    let rules = rules_and_weights.into_iter().map(|(r, _)| r).collect();
    Ok(RuleSet::new(rules, Either::List(weights), Box::new(function), contract))
}

fn err_checked<T>(option: Option<Vec<Result<T, String>>>) -> Result<Vec<T>, JsonError> {
    let vec = match option {
        Some(v) => v,
        None => { return Err(JsonError::MissingFieldError("JSON object: missing or invalid field 'rules'")); }
    };
    let mut ret = Vec::with_capacity(vec.capacity());
    for (index, value) in vec.into_iter().enumerate() {
        match value {
            Ok(thing) => { ret.push(thing); }
            Err(s) => {
                let string = format!("Rule object {}: {}", index, s);
                unsafe {
                    let static_ref = mem::transmute(&string as &str);
                    mem::forget(string);
                    return Err(JsonError::MissingFieldError(static_ref));
                }
            }
        }
    }
    Ok(ret)
}

fn read_file(path: &str) -> Result<String, JsonError> {
    let path = Path::new(path);
    let mut content = String::new();
    try!(File::open(&path).and_then(|mut f| f.read_to_string(&mut content)));
    return Ok(content);
}

fn get_contract(value: &Value) -> Result<GraphContract, JsonError> {
    if let Some(map) = value.as_object()
                            .and_then(|o| o.get("contract"))
                            .and_then(Value::as_object) {

        let mut contract = GraphContract::default();

        if let Some(thing) = map.get("isAcyclic") {
            match thing.as_boolean() {
                Some(b) => { contract.is_acyclic = b; },
                None => { return Err(JsonError::MissingFieldError("JSON object: invalid value for field 'contract': isAcyclic")); }
            }
        }
        if let Some(thing) = map.get("maxEdgesPerNode") {
            match thing.as_u64() {
                Some(u) => { contract.max_edges_per_node = Some(u as u32); },
                None => { return Err(JsonError::MissingFieldError("JSON object: invalid value for 'contract': maxEdgesPerNode")); }
            }
        }
        if let Some(thing) = map.get("isConnected") {
            match thing.as_boolean() {
                Some(b) => { contract.is_connected = b; },
                None => { return Err(JsonError::MissingFieldError("JSON object: invalid value for field 'contract': isConnected")); }
            }
        }
        Ok(contract)
    }
    else {
        Err(JsonError::MissingFieldError("JSON object: missing field 'contract'"))
    }
}

fn get_labels(value: &Value) -> (Vec<UntypedNode>, Vec<UntypedEdge>) {
    unimplemented!();
}

fn get_rules<S, T, U, V>(value: &Value) -> Option<Vec<Result<(Rule<S, T, U, V>, f32), String>>>
where S: SerSymbol, T: SerSymbol, U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    value.as_object()
         .and_then(|o| o.get("rules"))
         .and_then(Value::as_array)
         .map(|vec| vec.into_iter()
                       .map(Value::as_object)
                       .map(|rule| create_rule::<S, T, U, V>(rule))
                       .collect()
         )
}

fn create_rule<S, T, U, V>(map: Option<&BTreeMap<String, Value>>) -> Result<(Rule<S, T, U, V>, f32), String>
where S: SerSymbol, T: SerSymbol, U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    let weight = match map.and_then(|m| m.get("weight")) {
        Some(val) => val.as_f64(),
        None => Some(1.0)
    };
    let start = match map.and_then(|m| m.get("start"))
                         .and_then(Value::as_object) {
        Some(o) => parse_start::<S, T, U, V>(o),
        None => Err("field 'start' is invalid".to_string())
    };
    let result = match map.and_then(|m| m.get("result"))
                          .and_then(Value::as_object) {
        Some(o) => parse_result::<S, T>(o),
        None => Err("field 'result' is invalid".to_string())
    };
    let same = match map.and_then(|m| m.get("sameNodes")) {
        Some(val) => val.as_array().and_then(parse_same_nodes),
        None => if let Ok(g) = start.as_ref() {
            let mut m = HashMap::new();
            for i in 0..g.nodes() {
                m.insert(i, i);
            }
            Some(m)
        }
        else { None }
    };
    if start.is_ok() && result.is_ok() && weight.is_some() && same.is_some() {
        Ok((Rule::new(start.unwrap(), result.unwrap(), same.unwrap()), weight.unwrap() as f32))
    }
    else {
        let string = if weight.is_none() {
            "invalid value for field 'weight'".to_string()
        }
        else if let Err(s) = start {
            s
        }
        else if let Err(s) = result {
            s
        }
        else {
            "invalid value for field 'sameNodes'".to_string()
        };

        Err(string)
    }
}

fn parse_start<S, T, U, V>(map: &BTreeMap<String, Value>) -> Result<DirectedGraph<U, V>, String>
where U: SerSymbol + SymbolSet<S>, V: SerSymbol + SymbolSet<T> {
    let nodes = match map.get("nodes").and_then(Value::as_array) {
        Some(ns) => {
            let mut ret = Vec::new();
            for i in 0..ns.len() {
                let n = &ns[i];
                if let Some(arr) = n.as_array() {
                    ret.reserve_exact(arr.len());
                    match arr.get(0).and_then(Value::as_string) {
                        Some(s) => {
                            if let Some(node) = U::parse(s, &n) {
                                ret.push(node);
                            }
                            else { return Err(format!("node {} has invalid field", i)); }
                        },
                        _ => { return Err(format!("node {} has invalid field", i)); }
                    }
                }
                else { return Err(format!("node {} is invalid", i)); }
            }
            ret
        },
        None => { return Err("missing field 'nodes'".to_string()); }
    };
    let edges = match map.get("edges").and_then(Value::as_array) {
        Some(es) => {
            let mut ret = Vec::new();
            for i in 0..es.len() {
                let e = &es[i];
                if let Some(arr) = e.as_array() {
                    ret.reserve_exact(arr.len());
                    match (arr.get(0).and_then(Value::as_u64),
                           arr.get(1).and_then(Value::as_u64),
                           arr.get(2).and_then(Value::as_string)) {
                        (Some(begin), Some(end), Some(s)) => {
                            if let Some(edge) = V::parse(s, &e) {
                                ret.push((begin as usize, end as usize, edge));
                            }
                            else { return Err(format!("edge {} has invalid field", i)); }
                        },
                        _ => { return Err(format!("edge {} has invalid field", i)); }
                    }
                }
                else { return Err(format!("edge {} is invalid", i)); }
            }
            ret
        },
        None => { return Err("missing field 'edges'".to_string()); }
    };
    Ok(DirectedGraph::from_vec(&nodes, &edges))
}

fn parse_result<S, T>(map: &BTreeMap<String, Value>) -> Result<DirectedGraph<S, T>, String>
where S: SerSymbol, T: SerSymbol {
    let nodes = match map.get("nodes").and_then(Value::as_array) {
        Some(ns) => {
            let mut ret = Vec::new();
            for i in 0..ns.len() {
                let n = &ns[i];
                if let Some(arr) = n.as_array() {
                    ret.reserve_exact(arr.len());
                    match arr.get(0).and_then(Value::as_string) {
                        Some(s) => {
                             if let Some(node) = S::parse(s, arr.get(1).unwrap_or(&Value::Null)) {
                                 ret.push(node);
                             }
                             else { return Err(format!("node {} has invalid field", i)); }
                        },
                        _ => { return Err(format!("node {} has invalid field", i)); }
                    }
                }
                else { return Err(format!("node {} is invalid", i)); }
            }
            ret
        },
        None => { return Err("missing field 'nodes'".to_string()); }
    };
    let edges = match map.get("edges").and_then(Value::as_array) {
        Some(es) => {
            let mut ret = Vec::new();
            for i in 0..es.len() {
                let e = &es[i];
                if let Some(arr) = e.as_array() {
                    ret.reserve_exact(arr.len());
                    match (arr.get(0).and_then(Value::as_u64),
                           arr.get(1).and_then(Value::as_u64),
                           arr.get(2).and_then(Value::as_string)) {
                        (Some(begin), Some(end), Some(s)) => {;
                            if let Some(edge) = T::parse(s, &Value::Null) {
                                ret.push((begin as usize, end as usize, edge));
                            }
                            else { return Err(format!("edge {} has invalid field", i)); }
                        },
                        _ => { return Err(format!("edge {} has invalid field", i)); }
                    }
                }
                else { return Err(format!("edge {} is invalid", i)); }
            }
            ret
        },
        None => { return Err("missing field 'edges'".to_string()); }
    };
    Ok(DirectedGraph::from_vec(&nodes, &edges))
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
