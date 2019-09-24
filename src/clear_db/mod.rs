use std::path::Path;
use sled::Db;

pub fn clear_db(db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    match db.clear() {
        Ok(_) => println!("done"),
        Err(e) => println!("{}", e),
    }
}
