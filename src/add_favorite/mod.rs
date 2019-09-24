use sled::Db;
use std::path::Path;
use std::env;

use diar::util::print_done_if_ok;

pub fn add_to_diar(maybe_path_given: Option<&Path>, key: String, db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    match db.get(&key) {
        Ok(Some(_)) => {
            println!("already exist!");
        }
        _ => {
            match maybe_path_given {
                Some(path) => {
                    if path.exists() {
                        add_path_to_db(path, key, db);
                    } else {
                        println!("This path does not exist!: {}", path.to_str().unwrap());
                    }
                }
                None => {
                    add_current_to_db(key, db);
                }
            }
        }
    }
}

fn add_path_to_db(path: &Path, key: String, db: sled::Db) -> () {
     print_done_if_ok(db.insert(key, path.to_str().unwrap().as_bytes().to_vec()))
}

fn add_current_to_db(key: String, db: sled::Db) -> () {
    match env::current_dir() {
        Ok(current_path) => print_done_if_ok(db.insert(key, current_path.to_str().unwrap().as_bytes().to_vec())),
        Err(e) => println!("{}", e),
    }
}
