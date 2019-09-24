use std::path::Path;
use sled::Db;

// use diar::util::print_done_if_ok;

pub fn search_and_jump(user_input: String, db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    let matched_value = db.get(&user_input);
    match matched_value {
        Ok(Some(path)) => {
            let converted_str: String = String::from_utf8(path.to_vec()).unwrap();
            jump(Path::new(&converted_str));
        }

        _ => {
            let favorites = search(&user_input, db); 
            println!("Is this what you are jumping?");
            for (key, path) in favorites {
                println!(" key: {}, path: {}", key, path);
            }
        }

    }


}

fn search(searched_word: &str, db: sled::Db) -> Vec<(String, String)> {
    let iter_tree = db.iter();
    
    let mut favorites: Vec<(String, String)> = Vec::new();

    for t in iter_tree {
        let unwrapped_keyvalue = t.unwrap();
        let key: String = String::from_utf8(unwrapped_keyvalue.0.to_vec()).unwrap();
        let value: String = String::from_utf8(unwrapped_keyvalue.1.to_vec()).unwrap();
        favorites.push((key, value));
    }
    let fav_dir = favorites.into_iter().filter(|(k, _)| k.contains(searched_word)).collect::<Vec<(String, String)>>();
    fav_dir
}


fn jump(dest_dir: &Path) -> () {
    println!("{}", dest_dir.to_str().unwrap());
}

