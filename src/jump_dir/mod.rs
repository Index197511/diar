use sled::Db;
use std::path::Path;

type Favorite = (String, String);

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

fn get_favorites(iter_db: sled::Iter<'_>) -> Vec<Favorite> {
    let mut favorites: Vec<Favorite> = Vec::new();
    let favorites_utf8 = iter_db.filter(|result| result.is_ok()).map(|ok| ok.unwrap());

    for converted_favorite in favorites_utf8.map(|favorite_utf8| from_utf8s(favorite_utf8)) {
        if let Some(favorite) = converted_favorite {
            favorites.push(favorite);
        }
    }

    favorites
}

fn from_utf8s(favorite_ivec: (sled::IVec, sled::IVec)) -> Option<Favorite> {
    let key_utf8 = favorite_ivec.0.to_vec();
    let path_utf8 = favorite_ivec.1.to_vec();

    match (String::from_utf8(key_utf8), String::from_utf8(path_utf8)) {
        (Ok(key), Ok(path)) => Some((key, path)),
        _ => None,
    }
}
