pub type Favorite = (String, String);

pub enum JumpTo {
    Key(String),
    ProjectRoot,
}

pub enum GetProjectRootError {
    GitCommandNotFound,
    DotGitNotFound,
}
