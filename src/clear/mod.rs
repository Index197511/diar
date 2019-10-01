use sled::Db;
use std::path::Path;

use diar::util::print_done_if_ok;

pub fn clear_db(db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    print_done_if_ok(db.clear());
}
