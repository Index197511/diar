use crate::types::WhereToAdd;
use crate::{
    command::{print_result, CommandError, CommandResult},
    domain::model::Favorite,
};
use crate::{domain::repository::IRepository, error::error};
use std::env;
use std::fs;
use std::path::Path;

pub fn add_favorite<T: IRepository>(repo: T, key: String, path: WhereToAdd) -> anyhow::Result<()> {
    if repo.exists(&key)? {
        return Err(CommandError::GivenKeyIsAlreadyExists.into());
    }

    match path {
        WhereToAdd::Path(path) => add_given_path_to_db(repo, key, path),
        WhereToAdd::CurrentDirectory => add_current_path_to_db(repo, key),
    }
    Ok(())
}

fn add_path_to_db<T: IRepository>(repo: T, key: String, path: &Path) {
    let path_str = path.to_str().unwrap().to_string();

    print_result(
        repo.add(&Favorite::new(key.clone(), path_str)),
        CommandResult::Added(key, path.to_str().unwrap().to_owned()),
    );
}

fn add_given_path_to_db<T: IRepository>(repo: T, key: String, path: &Path) {
    if path.exists() {
        add_path_to_db(repo, key, fs::canonicalize(path).unwrap().as_path());
    } else {
        error(&format!(
            "This path does not exist: {}",
            path.to_str().unwrap()
        ));
    }
}

fn add_current_path_to_db<T: IRepository>(repo: T, key: String) {
    match env::current_dir() {
        Ok(current_path) => add_path_to_db(repo, key, &current_path),
        Err(e) => println!("{}", e),
    }
}
