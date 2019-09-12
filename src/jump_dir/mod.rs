extern crate sled;

use std::path::Path;
use sled::Db;

pub fn seace_and_jump(searched_word: &str, db_path: &Path) -> Vec<(String, String)> {
    let tree = Db::open(db_path).unwrap();
    let iter_tree = tree.iter();
    
    let mut fav_dir: Vec<(String, String)> = Vec::new();

    for t in iter_tree {
        let unwrapped_keyvalue = t.unwrap();
        let key: String = String::from_utf8(unwrapped_keyvalue.0.to_vec()).unwrap();
        let value: String = String::from_utf8(unwrapped_keyvalue.1.to_vec()).unwrap();
        fav_dir.push((key, value));
    }
    
    fav_dir
}
