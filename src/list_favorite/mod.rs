extern crate sled;

use std::path::Path;
use sled::Db;

pub fn list(db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    let iter_tree = tree.iter().keys();
    for t in iter_tree {
        let key_value = t.unwrap();
        println!("key: {}, path: {}", key_value[0], key_value[1]);
    }
}
