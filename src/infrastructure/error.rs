use derive_more::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Display)]
pub enum InfraError {
    #[display(fmt = "home directory not found")]
    HomeDirectoryNotFound,
    #[display(fmt = "favorites not found")]
    FavoritesNotFound,
}

#[derive(Debug, ThisError, Display)]
pub enum ProjectRootPathError {
    #[display(fmt = ".git directory not found")]
    DotGitNotFound,
    #[display(fmt = "git command not found")]
    GitCommandNotFound,
    #[display(fmt = "cannot convert path to String")]
    ConnotConvertToString,
}

#[derive(Debug, ThisError, Display)]
pub enum CurrentDirectoryPathError {
    #[display(fmt = "cannot access current directory")]
    CannotAccessCurrentDirectory,
    #[display(fmt = "cannot convert path to String")]
    ConnotConvertToString,
}
