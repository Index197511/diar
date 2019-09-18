extern crate sled;

use std::path::Path;
use sled::Db;

pub fn search_and_jump(user_input: String, db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    let some_value = tree.get(&user_input);
    println!("jump");
    match some_value {
        Ok(Some(path)) => {
            let converted_str: String = String::from_utf8(path.to_vec()).unwrap();
            jump(Path::new(&converted_str));
        }

        _ => {
           println!("Not found") 
        }
    }


}

#[warn(dead_code)]
fn search(searched_word: &str, db_path: &Path) -> Vec<(String, String)> {
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


fn jump(dest_dir: &Path) -> () {
    println!("{}", dest_dir.to_str().unwrap());
}

#[warn(dead_code)]
fn read<T: std::str::FromStr> () -> T {
    let mut l = String::new();
    std::io::stdin().read_line(&mut l).ok();
    l.trim().parse().ok().unwrap()
}
