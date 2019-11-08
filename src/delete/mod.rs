use sled::Db;
use std::path::Path;

use diar::types::CommandName;
use diar::util::print_result;

pub fn delete_from_db(key: String, db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    match db.get(&key) {
        Ok(Some(p)) => {
            let path_string: String = String::from_utf8(p.to_vec()).unwrap();
            print_result(db.remove(&key), CommandName::Deleted((key, path_string)));
        }
        _ => println!("This key does not exist!: {}", key),
    };
}
