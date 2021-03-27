use diar::domain::model::Command;

#[test]
fn test_from_str() {
    use std::str::FromStr;

    assert_eq!(Command::from_str("add").unwrap(), Command::Add);
}
