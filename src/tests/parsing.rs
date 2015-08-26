use deserialization::json;

#[test]
fn parse_ruleset() {
    let value = json::read_value("src/tests/ruleset.json").unwrap();
    json::mission_rules(&value).unwrap();
}
