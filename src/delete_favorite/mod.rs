extern crate sled;

use std::path::Path;
use sled::Db;

pub fn delete_from_db(key: &str, db_path: &Path) {
    let tree = Db::open(db_path).unwrap();
    tree.del(key);
}
