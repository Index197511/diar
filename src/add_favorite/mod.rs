extern crate sled;

use sled::Db;
use std::path::Path;
use std::env;

pub fn add_to_db(path: &Path, key: String, db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    match tree.get(&key) {
        Ok(Some(_p)) => {
            println!("already exist!");
        }
        _ => {
            if path.exists() {
                let _ = tree.insert(key, path.to_str().unwrap().as_bytes().to_vec());
                println!("done");
            } else {
                println!("This path does not exist!: {}", path.to_str().unwrap());
            }
        }
    }
}
