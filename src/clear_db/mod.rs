extern crate sled;

use std::path::Path;
use sled::Db;

pub fn clear_db(db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    let _ = tree.clear();
}
