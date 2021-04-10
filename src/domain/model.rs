use strum_macros::EnumString;

#[derive(Clone)]
pub struct Favorite {
    name: String,
    path: String,
}

impl Favorite {
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Favorite {
        Favorite {
            name: name.into(),
            path: path.into(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
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
