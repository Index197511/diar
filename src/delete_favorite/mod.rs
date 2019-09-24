use std::path::Path;
use sled::Db;

use diar::util::print_done_if_ok;

pub fn delete_from_db(key: &str, db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    match db.get(key) {
        Ok(Some(_)) => print_done_if_ok(db.remove(key)),
        _ => println!("This key does not exist!: {}", key),
    };
}
