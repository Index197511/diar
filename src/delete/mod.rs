use sled::Db;

use diar::command::{CommandResult, print_result};
use diar::error::print_error;

pub fn delete_from_db(db: Db, key: String) {
    match db.get(&key) {
        Ok(Some(p)) => {
            let path_string: String = String::from_utf8(p.to_vec()).unwrap();
            print_result(db.remove(&key), CommandResult::Deleted(key, path_string));
        }
        _ => print_error(&format!("This key does not exist!: {}", key)),
    };
}
