use crate::command::WhereToAdd;
use crate::domain::repository::IRepository;
use crate::{command::CommandError, domain::model::Favorite};
use std::env;
use std::fs;
use std::path::Path;

pub fn add_favorite<T: IRepository>(
    repo: T,
    key: String,
    path: WhereToAdd,
) -> anyhow::Result<Favorite> {
    if repo.exists(&key).unwrap() {
        return Err(CommandError::GivenKeyIsAlreadyExists.into());
    }

    match path {
        WhereToAdd::Path(path) => add_given_path_to_db(repo, key, path),
        WhereToAdd::CurrentDirectory => add_current_path_to_db(repo, key),
    }
}

fn add_given_path_to_db<T: IRepository>(
    repo: T,
    key: String,
    path: &Path,
) -> anyhow::Result<Favorite> {
    if !path.exists() {
        return Err(CommandError::PathNotFound.into());
    }

    let favorite = Favorite::new(
        key,
        fs::canonicalize(path)
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_owned(),
    );

    match repo.add(&favorite) {
        Ok(_) => Ok(favorite),
        Err(e) => Err(e),
    }
}

fn add_current_path_to_db<T: IRepository>(repo: T, key: String) -> anyhow::Result<Favorite> {
    match env::current_dir() {
        Ok(current_path) => {
            let favorite = Favorite::new(key, current_path.as_path().to_str().unwrap().to_owned());
            match repo.add(&favorite) {
                Ok(_) => Ok(favorite),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e.into()),
    }
}
