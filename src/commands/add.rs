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
    if repo.exists(&key)? {
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
        fs::canonicalize(path)?
            .as_path()
            .to_str()
            .unwrap()
            .to_owned(),
    );

    repo.add(&favorite).map(|_| Ok(favorite))?
}

fn add_current_path_to_db<T: IRepository>(repo: T, key: String) -> anyhow::Result<Favorite> {
    match env::current_dir() {
        Ok(current_path) => {
            let favorite: anyhow::Result<Favorite> = current_path
                .as_path()
                .to_str()
                .map(|path| Favorite::new(key, path.to_owned()))
                .ok_or_else(|| CommandError::InvalidPath.into());
            repo.add(&favorite?)
        }
        Err(e) => Err(e.into()),
    }
}
