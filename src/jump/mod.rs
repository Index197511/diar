extern crate skim;

use diar::error::{error, suggest, GetProjectRootFailed};
use diar::types::JumpTo;
use diar::util::{get_favorites, search};
use skim::prelude::*;
use sled::Db;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;

pub fn jump_to(db: Db, to: JumpTo) {
    match to {
        JumpTo::Key(key) => jump_to_key(db, &key),
        JumpTo::ProjectRoot => jump_to_project_root(),
        JumpTo::FuzzyFinder => jump_with_skim(db),
    }
}

fn jump_with_skim(db: Db) {
    let skim_option = SkimOptionsBuilder::default()
        .height(Some("30%"))
        .multi(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let favorites = get_favorites(db)
        .iter()
        .map(|(key, path)| format!("{} -> {}", key, path))
        .collect::<Vec<String>>();
    let items = item_reader.of_bufread(Cursor::new(favorites.join("\n")));

    let selected_items = Skim::run_with(&skim_option, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items {
        println!(
            "{}",
            item.output()
                .into_owned()
                .split(" -> ")
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
        );
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

fn get_project_root_path() -> Result<String, GetProjectRootFailed> {
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
                Err(GetProjectRootFailed::DotGitNotFound)
            }
        }
        Err(_) => Err(GetProjectRootFailed::GitCommandNotFound),
    }
}

fn jump_to_project_root() {
    match get_project_root_path() {
        Ok(path_string) => jump(Path::new(&path_string)),
        Err(GetProjectRootFailed::DotGitNotFound) => {
            error(".git directory not found.");
        }
        Err(GetProjectRootFailed::GitCommandNotFound) => {
            error("Command 'git' not found.");
        }
    }
}

fn jump(to: &Path) {
    println!("{}", to.to_str().unwrap());
}
