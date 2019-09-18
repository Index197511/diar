extern crate sled;

use std::path::Path;
use sled::Db;

pub fn delete_from_db(key: &str, db_path: &Path) {
    let tree = Db::open(db_path).unwrap();
    match tree.get(key) {
        Ok(Some(_p)) => {
            let _ = tree.remove(key);
            println!("done");
        }
        _ => println!("This key does not exist!: {}", key)
    };
}
