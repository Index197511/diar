use sled::Db;

use diar::types::CommandResult;
use diar::util::print_result;

pub fn clear_db(db: Db) {
    print_result(db.clear(), CommandResult::Cleared);
}
