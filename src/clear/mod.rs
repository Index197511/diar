use sled::Db;

use diar::command::{CommandResult, print_result};

pub fn clear_db(db: Db) {
    print_result(db.clear(), CommandResult::Cleared);
}
