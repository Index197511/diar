use sled::Db;

use crate::command::{print_result, CommandResult};
use crate::error::error;

pub fn rename_favorite(db: Db, old_key: String, new_key: String) {
    match db.get(&old_key) {
        Ok(Some(path)) => {
            print_result(
                db.remove(&old_key)
                    .and_then(|_| db.insert(&new_key, path.to_vec())),
                CommandResult::Renamed(old_key, new_key),
            );
        }
        _ => {
            error(&format!("This key does not exist: {}", old_key));
        }
    }
}
