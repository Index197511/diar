use sled::Db;
use std::path::Path;

use diar::util::get_favorites;
use diar::types::Favorite;

pub fn jump_if_matched(user_input: String, db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    let maybe_path_matched = db.get(&user_input);

    match maybe_path_matched {
        Ok(Some(path)) => {
            let path_string: String = String::from_utf8(path.to_vec()).unwrap();
            jump(Path::new(&path_string));
        }
        _ => {
            suggest(search(&user_input, db));
        }
    }
}

fn jump(dest_dir: &Path) -> () {
    println!("{}", dest_dir.to_str().unwrap());
}

fn suggest(searched: Vec<Favorite>) -> () {
    println!("Is this what you are jumping?");
    for (key, path) in searched {
        println!("       {} -> {}", key, path);
    }
}

fn search(searched_word: &str, db: sled::Db) -> Vec<Favorite> {
    let iter_db = db.iter();
    let favorites = get_favorites(iter_db);

    favorites
        .into_iter()
        .filter(|(key, _)| key.contains(searched_word))
        .collect::<Vec<Favorite>>()
}
