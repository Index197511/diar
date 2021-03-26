use derive_new::new;
use getset::Getters;
use strum_macros::EnumString;

#[derive(new, Getters)]
pub struct Favorite {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    path: String,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Add,
    Delete,
    Rename,
    List,
    Jump,
    Clear,
    Ls,
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    assert_eq!(Command::from_str("add").unwrap(), Command::Add);
}
