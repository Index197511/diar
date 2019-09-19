extern crate sled;

use std::path::Path;
use sled::Db;

pub fn search_and_jump(user_input: String, db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    let some_value = tree.get(&user_input);
    match some_value {
        Ok(Some(path)) => {
            let converted_str: String = String::from_utf8(path.to_vec()).unwrap();
            jump(Path::new(&converted_str));
        }

        _ => {
            let fav_dirs = search(&user_input, tree); 
            println!("Is this what you are jumping?");
            for t in fav_dirs {
                println!(" key: {}, path: {}", t.0, t.1);
            }
        }

    }


}

#[warn(dead_code)]
fn search(searched_word: &str, tree: sled::Db) -> Vec<(String, String)> {
    let iter_tree = tree.iter();
    
    let mut fav_dir: Vec<(String, String)> = Vec::new();

    for t in iter_tree {
        let unwrapped_keyvalue = t.unwrap();
        let key: String = String::from_utf8(unwrapped_keyvalue.0.to_vec()).unwrap();
        let value: String = String::from_utf8(unwrapped_keyvalue.1.to_vec()).unwrap();
        fav_dir.push((key, value));
    }
    let fav_dir = fav_dir.into_iter().filter(|(k, _v)| k.contains(searched_word)).collect::<Vec<(String, String)>>();
    fav_dir
}


fn jump(dest_dir: &Path) -> () {
    println!("{}", dest_dir.to_str().unwrap());
}

