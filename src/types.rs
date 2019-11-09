pub type Favorite = (String, String);

pub enum JumpTo {
    Key(String),
    ProjectRoot,
}

pub enum GetProjectRootError {
    GitCommandNotFound,
    DotGitNotFound,
}

pub enum CommandResult {
    Added(Favorite),
    Deleted(Favorite),
    Cleared,
    Renamed(String, String),
}
