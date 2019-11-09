use sled::Db;

use diar::types::CommandResult;
use diar::util::print_result;

pub fn delete_from_db(db: Db, key: String) {
    match db.get(&key) {
        Ok(Some(p)) => {
            let path_string: String = String::from_utf8(p.to_vec()).unwrap();
            print_result(db.remove(&key), CommandResult::Deleted((key, path_string)));
        }
        _ => println!("This key does not exist!: {}", key),
    };
}
