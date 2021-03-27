use diar::domain::model::{Command, Favorite};

#[test]
fn test_favorite_getters() {
    let name = "name1".to_string();
    let path = "path1".to_string();
    let favorite = Favorite::new(&name, &path);

    assert_eq!(favorite.name(), name);
    assert_eq!(favorite.path(), path);
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    assert_eq!(Command::from_str("add"), Ok(Command::Add));
    assert_eq!(Command::from_str("delete"), Ok(Command::Delete));
    assert_eq!(Command::from_str("rename"), Ok(Command::Rename));
    assert_eq!(Command::from_str("list"), Ok(Command::List));
    assert_eq!(Command::from_str("jump"), Ok(Command::Jump));
    assert_eq!(Command::from_str("clear"), Ok(Command::Clear));
    assert_eq!(Command::from_str("ls"), Ok(Command::Ls));

    assert!(Command::from_str("").is_err());
}
