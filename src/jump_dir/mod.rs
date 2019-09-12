extern crate sled;

use std::path::Path;
use sled::Db;

pub fn search(db_path: &Path, searched_word: &str) -> Vec<(String, String)> {
    let tree = Db::open(db_path).unwrap();
    let iter_tree = tree.iter();
    
    let mut fav_dir: Vec<(String, String)> = Vec::new();

    for t in iter_tree {
        let unwrapped_keyvalue = t.unwrap();
        let key: String = String::from_utf8(unwrapped_keyvalue.0.to_vec()).unwrap();
        let value: String = String::from_utf8(unwrapped_keyvalue.1.to_vec()).unwrap();
        fav_dir.push((key, value));
    }
    let fav_dir = fav_dir.into_iter().filter(|(_k, v)| v.contains(searched_word)).collect::<Vec<(String, String)>>();
    fav_dir
}


pub fn jump(dest_dir: &Path) -> () {
    println!("{}", dest_dir.to_str().unwrap());
}
