use std::path::Path;

pub type Favorite = (String, String);

pub enum JumpTo {
    Key(String),
    ProjectRoot,
}

pub enum WhereToAdd<'a> {
    Path(&'a Path),
    CurrentDirectory,
}
