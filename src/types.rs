use std::path::Path;

pub enum JumpTo {
    Key(String),
    ProjectRoot,
    FuzzyFinder,
}

pub enum WhereToAdd<'a> {
    Path(&'a Path),
    CurrentDirectory,
}
