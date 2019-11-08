use sled::Db;
use std::env;
use std::fs;
use std::path::Path;

use diar::types::CommandName;
use diar::util::print_result;

pub fn add_favorite(maybe_path_given: Option<&Path>, key: String, db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    match db.get(&key) {
        Ok(Some(_)) => {
            println!("already exist!");
        }
        _ => match maybe_path_given {
            Some(path) => {
                if path.exists() {
                    add_path_to_db(fs::canonicalize(path).unwrap().as_path(), key, db);
                } else {
                    println!("This path does not exist!: {}", path.to_str().unwrap());
                }
            }
            None => {
                add_current_path_to_db(key, db);
            }
        },
    }
}

fn add_path_to_db(path: &Path, key: String, db: sled::Db) {
    print_result(
        db.insert(&key, path.to_str().unwrap().as_bytes().to_vec()),
        CommandName::Added((key, path.to_str().unwrap().to_owned())),
    );
}

fn add_current_path_to_db(key: String, db: sled::Db) {
    match env::current_dir() {
        Ok(current_path) => {
            print_result(
                db.insert(&key, current_path.to_str().unwrap().as_bytes().to_vec()),
                CommandName::Added((key, current_path.to_str().unwrap().to_owned())),
            );
        }
        Err(e) => println!("{}", e),
    }
}
