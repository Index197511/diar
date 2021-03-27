extern crate skim;

use crate::types::JumpTo;
use crate::util::search;
use crate::{
    domain::repository::IRepository,
    error::{error, suggest, GetProjectRootFailed},
};
use skim::prelude::*;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;

pub fn jump_to<T: IRepository>(repo: T, to: JumpTo) {
    match to {
        JumpTo::Key(key) => jump_to_key(repo, &key),
        JumpTo::ProjectRoot => jump_to_project_root(),
        JumpTo::FuzzyFinder => jump_with_skim(repo),
    }
}

fn jump_with_skim<T: IRepository>(repo: T) {
    let skim_option = SkimOptionsBuilder::default()
        .height(Some("30%"))
        .multi(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();

    let favorites = repo
        .get_all()
        .unwrap()
        .iter()
        .map(|favorite| format!("{} -> {}", favorite.name(), favorite.path()))
        .collect::<Vec<String>>();
    let items = item_reader.of_bufread(Cursor::new(favorites.join("\n")));

    let selected_items = Skim::run_with(&skim_option, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

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

fn jump_to_key<T: IRepository>(repo: T, key: &str) {
    let maybe_path_matched = repo.get(key);

    match maybe_path_matched {
        Ok(Some(favorite)) => {
            jump(Path::new(favorite.path()));
        }
        _ => {
            suggest(key, search(key, repo));
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
