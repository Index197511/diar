use sled::Db;

use diar::command::{print_result, CommandResult};
use diar::error::print_error;

pub fn rename_favorite(db: Db, old_key: String, new_key: String) {
    match db.get(&old_key) {
        Ok(Some(path)) => {
            db.remove(&old_key).unwrap();
            print_result(
                db.insert(&new_key, path.to_vec()),
                CommandResult::Renamed(old_key, new_key),
            );
        }
        _ => {
            print_error(&format!("This key does not exist: {}", old_key));
        }
    }
}
