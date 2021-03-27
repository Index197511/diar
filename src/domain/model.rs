use derive_new::new;
use strum_macros::EnumString;

#[derive(new, Clone)]
pub struct Favorite {
    name: String,
    path: String,
}

impl Favorite {
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
