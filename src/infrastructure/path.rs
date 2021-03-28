use std::{env, process::Command};

use super::error::{CurrentDirectoryPathError, ProjectRootPathError};

pub fn current_path() -> Result<String, CurrentDirectoryPathError> {
    match env::current_dir() {
        Ok(current_path) => match current_path.to_str() {
            Some(path) => Ok(path.to_owned()),
            None => Err(CurrentDirectoryPathError::ConnotConvertToString),
        },
        Err(_) => Err(CurrentDirectoryPathError::CannotAccessCurrentDirectory),
    }
}

pub fn project_root_path() -> Result<String, ProjectRootPathError> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --show-toplevel")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                match String::from_utf8(output.stdout) {
                    Ok(path) => Ok(path.trim_end().to_string()),
                    Err(_) => Err(ProjectRootPathError::ConnotConvertToString),
                }
            } else {
                Err(ProjectRootPathError::DotGitNotFound)
            }
        }
        Err(_) => Err(ProjectRootPathError::GitCommandNotFound),
    }
}
