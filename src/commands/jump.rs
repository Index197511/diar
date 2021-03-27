use crate::domain::{repository::IRepository, service::search};
use crate::interface::presenter::suggest;
use crate::{command::CommandError, domain::model::Favorite};
use skim::prelude::*;
use std::io::Cursor;
use std::process::Command;

pub enum JumpTo {
    Key(String),
    ProjectRoot,
    FuzzyFinder,
}

pub fn jump_to<T: IRepository>(repo: &T, to: JumpTo) -> anyhow::Result<String> {
    match to {
        JumpTo::FuzzyFinder => jump_with_skim(repo).map(|fav| fav.path()),
        JumpTo::Key(key) => jump_to_key(repo, &key).map(|fav| fav.path()),
        JumpTo::ProjectRoot => jump_to_project_root(),
    }
}

fn jump_with_skim<T: IRepository>(repo: &T) -> anyhow::Result<Favorite> {
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

    let selected_items = Skim::run_with(&skim_option, Some(items)).map(|out| out.selected_items);

    let skim_error = Err(CommandError::SkimErrorOccured.into());
    match selected_items {
        Some(item) if !item.is_empty() => {
            let mut favorite = item
                .get(0)
                .unwrap()
                .output()
                .into_owned()
                .split(" -> ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            match (favorite.pop(), favorite.pop()) {
                (Some(path), Some(key)) => Ok(Favorite::new(key, path)),
                _ => skim_error,
            }
        }
        _ => Err(CommandError::SkimErrorOccured.into()),
    }
}

fn jump_to_key<T: IRepository>(repo: &T, key: &str) -> anyhow::Result<Favorite> {
    let maybe_path_matched = repo.get(key);

    match maybe_path_matched {
        Ok(Some(favorite)) => Ok(favorite),
        _ => {
            let favorites = repo.get_all()?;
            suggest(key, search(key, favorites));
            Err(CommandError::GivenKeyNotFound.into())
        }
    }
}

fn get_project_root_path() -> anyhow::Result<String> {
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
                Err(CommandError::DotGitNotFound.into())
            }
        }
        Err(_) => Err(CommandError::GitCommandNotFound.into()),
    }
}

fn jump_to_project_root() -> anyhow::Result<String> {
    match get_project_root_path() {
        Ok(path) => Ok(path),
        Err(e) => Err(e),
    }
}
