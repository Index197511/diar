pub type Favorite = (String, String);

pub enum JumpTo {
    Key(String),
    ProjectRoot,
}

pub enum GetProjectRootError {
    GitCommandNotFound,
    DotGitNotFound,
}

pub enum CommandName {
    Added(Favorite),
    Deleted(Favorite),
    Cleared,
    Renamed(String, String),
}
