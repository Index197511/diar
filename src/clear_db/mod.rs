use std::path::Path;
use sled::Db;

pub fn clear_db(db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    match tree.clear() {
        Ok(_) => println!("done"),
        Err(e) => println!("{}", e),
    }
}
