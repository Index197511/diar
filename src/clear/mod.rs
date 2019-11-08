use sled::Db;
use std::path::Path;

use diar::types::CommandName;
use diar::util::print_result;

pub fn clear_db(db_path: &Path) {
    let db = Db::open(db_path).unwrap();
    print_result(db.clear(), CommandName::Cleared);
}
