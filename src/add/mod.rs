use sled::Db;
use std::env;
use std::fs;
use std::path::Path;

use diar::command::{CommandResult, print_result};
use diar::error::print_error;

pub fn add_favorite(db: Db, key: String, path: Option<&Path>) {
    match db.get(&key) {
        Ok(Some(_)) => {
            print_error("This path already exist!");
        }
        _ => match path {
            Some(path) => add_given_path_to_db(db, key, path),
            None => add_current_path_to_db(db, key),
        },
    }
}

fn add_path_to_db(db: Db, key: String, path: &Path) {
    print_result(
        db.insert(&key, path.to_str().unwrap().as_bytes().to_vec()),
        CommandResult::Added(key, path.to_str().unwrap().to_owned()),
    );
}

fn add_given_path_to_db(db: Db, key: String, path: &Path) {
    if path.exists() {
        add_path_to_db(db, key, fs::canonicalize(path).unwrap().as_path());
    } else {
        print_error(&format!(
            "This path does not exist: {}",
            path.to_str().unwrap()
        ));
    }
}

fn add_current_path_to_db(db: Db, key: String) {
    match env::current_dir() {
        Ok(current_path) => add_path_to_db(db, key, &current_path),
        Err(e) => println!("{}", e),
    }
}
