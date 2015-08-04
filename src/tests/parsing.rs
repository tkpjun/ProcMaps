use deserialization::json;

#[test]
fn parse_ruleset() {
    let value = json::read_value("src/tests/ruleset.json");
    if let Err(err) = value.as_ref() {
        println!("{}", err);
    }
    assert!(value.is_ok());

    let ruleset = json::mission_rules(&value.unwrap());
    if let Err(err) = ruleset.as_ref() {
        println!("{}", err);
    }
    assert!(ruleset.is_ok());
}
