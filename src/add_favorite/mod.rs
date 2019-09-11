extern crate sled;

use sled::Db;
use std::path::Path;

pub fn add_to_db(path: &Path, key: String, db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    tree.set(key, path.to_str().unwrap());

}
