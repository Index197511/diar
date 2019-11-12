use std::path::Path;

pub type Key = String;
pub type Favorite = (Key, String);

pub enum JumpTo {
    Key(Key),
    ProjectRoot,
}

pub enum WhereToAdd<'a> {
    Path(&'a Path),
    CurrentDirectory,
}
