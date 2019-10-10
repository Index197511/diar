use sled::Db;
use std::path::Path;

use diar::util::print_done_if_ok;

pub fn rename_favorite(old_key: String, new_key: String, db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    match db.get(&old_key) {
        Ok(Some(path)) => {
            db.remove(old_key).unwrap();
            print_done_if_ok(db.insert(new_key, path.to_vec()));
        }
        _ => {
            println!("This key does not exist: {}", old_key);
        }
    }
}
