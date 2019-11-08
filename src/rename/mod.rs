use sled::Db;
use std::path::Path;

use diar::types::CommandName;
use diar::util::print_result;

pub fn rename_favorite(old_key: String, new_key: String, db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    match db.get(&old_key) {
        Ok(Some(path)) => {
            db.remove(&old_key).unwrap();
            print_result(
                db.insert(&new_key, path.to_vec()),
                CommandName::Renamed(old_key, new_key),
            );
        }
        _ => {
            println!("This key does not exist: {}", old_key);
        }
    }
}
