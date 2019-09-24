use std::path::Path;
use sled::Db;

pub fn list(db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    let iter_tree = tree.iter();
    for t in iter_tree {
        let unwrapped_keyvalue = t.unwrap();
        let key: String = String::from_utf8(unwrapped_keyvalue.0.to_vec()).unwrap();
        let value: String = String::from_utf8(unwrapped_keyvalue.1.to_vec()).unwrap();

        println!("key: {}, path: {}", key, value);

    }
}
