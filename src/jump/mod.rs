use diar::types::JumpTo;
use diar::util::search;
use diar::error::{GetProjectRootError, suggest, print_error};
use sled::Db;
use std::path::Path;
use std::process::Command;

pub fn jump_to(db: Db, to: JumpTo) {
    match to {
        JumpTo::Key(key) => jump_to_key(db, &key),
        JumpTo::ProjectRoot => jump_to_project_root(),
    }
}

fn jump_to_key(db: Db, key: &str) {
    let maybe_path_matched = db.get(key);

    match maybe_path_matched {
        Ok(Some(path)) => {
            let path_string: String = String::from_utf8(path.to_vec()).unwrap();
            jump(Path::new(&path_string));
        }
        _ => {
            suggest(key, search(key, db));
        }
    }
}

fn get_project_root_path() -> Result<String, GetProjectRootError> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --show-toplevel")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout)
                    .unwrap()
                    .trim_end()
                    .to_string())
            } else {
                Err(GetProjectRootError::DotGitNotFound)
            }
        }
        Err(_) => Err(GetProjectRootError::GitCommandNotFound),
    }
}

fn jump_to_project_root() {
    match get_project_root_path() {
        Ok(path_string) => jump(Path::new(&path_string)),
        Err(GetProjectRootError::DotGitNotFound) => {
            print_error(".git directory not found.");
        }
        Err(GetProjectRootError::GitCommandNotFound) => {
            print_error("Command 'git' not found.");
        }
    }
}

fn jump(dest_dir: &Path) {
    println!("{}", dest_dir.to_str().unwrap());
}
