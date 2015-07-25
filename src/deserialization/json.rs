use super::runtime_labels::{NodeLabel, EdgeLabel};
use graph_grammar::rule::Rule;
use graph_grammar::labels::SearchLabel;
use graph_grammar::labels::Symbol;
use serde::json::{self, Value};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    return content;
}

pub fn deser_grammar(text: &str) ->
        (Vec<NodeLabel>, Vec<EdgeLabel>,
            Vec<Rule<NodeLabel, EdgeLabel, SearchLabel<NodeLabel>, SearchLabel<EdgeLabel>>>) {

    let data = match json::from_str(text) {
        Ok(value) => value,
        Err(why) => panic!("Problem deserializing data: {}", Error::description(&why)),
    };
    panic!("TODO");
}

pub fn deser_rules<T: Symbol, S: Symbol>(text: &str) ->
        Vec<Rule<T, S, SearchLabel<T>, SearchLabel<S>>> {

    let data = match json::from_str(text) {
        Ok(value) => value,
        Err(why) => panic!("Problem deserializing data: {}", Error::description(&why)),
    };
    panic!("TODO");
}
