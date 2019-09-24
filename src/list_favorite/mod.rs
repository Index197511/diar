use std::path::Path;
use sled::Db;

use diar::util::get_favorites;

pub fn list(db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    let iter_db = db.iter() ;

    for (key, path) in get_favorites(iter_db) {
        println!("{} -> {}", key, path);

    }
}
