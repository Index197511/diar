use std::path::Path;
use sled::Db;

pub fn delete_from_db(key: &str, db_path: &Path) {
    let tree = Db::open(db_path).unwrap();
    match tree.get(key) {
        Ok(Some(_p)) => {
            tree.remove(key);
        }
        _ => println!("This key does not exist!: {}", key)
    };
}
