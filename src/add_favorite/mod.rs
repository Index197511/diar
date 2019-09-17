extern crate sled;

use sled::Db;
use std::path::Path;

pub fn add_to_db(path: &Path, key: String, db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    if path.exists() {
        tree.insert(key, path.to_str().unwrap().as_bytes().to_vec());
    } else {
        println!("This path does not exist!: {}", path.to_str().unwrap());
    }
}
