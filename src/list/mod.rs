use sled::Db;

use diar::util::get_favorites;

pub fn list_favorites(db: Db) {
    let iter_db = db.iter();

    for (key, path) in get_favorites(iter_db) {
        println!("        {} -> {}", key, path);
    }
}
