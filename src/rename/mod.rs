use sled::Db;

use diar::types::CommandResult;
use diar::util::print_result;

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
            println!("This key does not exist: {}", old_key);
        }
    }
}
