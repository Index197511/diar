use super::error::{CurrentDirectoryPathError, ProjectRootPathError};
use crate::domain::command_line::ICommandLineHandler;
use anyhow::Result;
use std::{env, fs, path::Path, process::Command};

pub struct CommandLineHandler;

impl ICommandLineHandler for CommandLineHandler {
    fn current_path() -> Result<String> {
        match env::current_dir() {
            Ok(current_path) => match current_path.to_str() {
                Some(path) => Ok(path.to_owned()),
                None => Err(CurrentDirectoryPathError::ConnotConvertToString.into()),
            },
            Err(_) => Err(CurrentDirectoryPathError::CannotAccessCurrentDirectory.into()),
        }
    }

    fn project_root_path() -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("git rev-parse --show-toplevel")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    match String::from_utf8(output.stdout) {
                        Ok(path) => Ok(path.trim_end().to_string()),
                        Err(_) => Err(ProjectRootPathError::ConnotConvertToString.into()),
                    }
                } else {
                    Err(ProjectRootPathError::DotGitNotFound.into())
                }
            }
            Err(_) => Err(ProjectRootPathError::GitCommandNotFound.into()),
        }
    }

    fn be_absolute_from(path_str: &str) -> Option<String> {
        let path = Path::new(path_str);
        if path.is_absolute() {
            return Some(path.to_str().to_owned()?.to_string());
        }

        Some(
            fs::canonicalize(Path::new(path))
                .ok()?
                .as_path()
                .to_str()?
                .to_owned(),
        )
    }
}
