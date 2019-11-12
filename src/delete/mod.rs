use sled::Db;

use diar::command::{CommandResult, print_result};
use diar::error::error;

pub fn delete_from_db(db: Db, key: String) {
    match db.get(&key) {
        Ok(Some(path)) => {
            let path_string: String = String::from_utf8(path.to_vec()).unwrap();
            print_result(db.remove(&key), CommandResult::Deleted(key, path_string));
        }
        _ => error(&format!("This key does not exist!: {}", key)),
    };
}
