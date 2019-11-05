use sled::Db;
use std::path::Path;
use std::process::Command;

use diar::types::{Favorite, GetProjectRootError, JumpTo};
use diar::util::get_favorites;

pub fn jump_to(to: JumpTo, db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    match to {
        JumpTo::Key(key) => jump_to_key(&key, db),
        JumpTo::ProjectRoot => jump_to_project_root(),
    }
}

fn jump_to_key(key: &str, db: sled::Db) {
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
            println!("Error: .git directory not found.");
        }
        Err(GetProjectRootError::GitCommandNotFound) => {
            println!("Error: Command 'git' not found.");
        }
    }
}

fn jump(dest_dir: &Path) {
    println!("{}", dest_dir.to_str().unwrap());
}

fn suggest(input: &str, searched: Vec<Favorite>) {
    println!("Error: Key '{}' not found.\n", input);
    println!("Is this what you are jumping?");
    for (key, path) in searched {
        println!("       {} -> {}", key, path);
    }
}

fn search(searched_word: &str, db: sled::Db) -> Vec<Favorite> {
    let iter_db = db.iter();
    let favorites = get_favorites(iter_db);

    favorites
        .into_iter()
        .filter(|(key, _)| key.contains(searched_word))
        .collect::<Vec<Favorite>>()
}
