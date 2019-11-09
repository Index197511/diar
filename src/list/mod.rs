use sled::Db;

use diar::util::{get_favorites, print_favorites};

pub fn list_favorites(db: Db) {
    print_favorites(get_favorites(db.iter()))
}
