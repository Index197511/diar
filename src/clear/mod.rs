use sled::Db;

use diar::command::{print_result, CommandResult};

pub fn clear_db(db: Db) {
    print_result(db.clear(), CommandResult::Cleared);
}
