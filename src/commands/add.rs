use crate::command::WhereToAdd;
use crate::domain::repository::IRepository;
use crate::interface::presenter::{error, print_result};
use crate::{
    command::{CommandError, CommandResult},
    domain::model::Favorite,
};
use std::env;
use std::fs;
use std::path::Path;

pub fn add_favorite<T: IRepository>(repo: T, key: String, path: WhereToAdd) {
    if repo.exists(&key).unwrap() {
        return error(&CommandError::GivenKeyIsAlreadyExists.to_string());
    }

    match path {
        WhereToAdd::Path(path) => add_given_path_to_db(repo, key, path),
        WhereToAdd::CurrentDirectory => add_current_path_to_db(repo, key),
    }
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
