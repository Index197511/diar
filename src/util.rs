extern crate sled;

use super::types::Favorite;
use dirs::home_dir;
use sled::Db;

pub fn get_favorites(db: Db) -> Vec<Favorite> {
    let mut favorites: Vec<Favorite> = Vec::new();
    let favorites_utf8 = db
        .iter()
        .filter(|maybe_favorite| maybe_favorite.is_ok())
        .map(|ok_favorite| ok_favorite.unwrap());

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

pub fn print_favorites(favorites: Vec<Favorite>) {
    for (key, path) in favorites {
        println!("       {} -> {}", key, path);
    }
}

pub fn search(searched_word: &str, db: Db) -> Vec<Favorite> {
    let favorites = get_favorites(db);

    favorites
        .into_iter()
        .filter(|(key, _)| key.contains(searched_word))
        .collect::<Vec<Favorite>>()
}

pub fn generate_path_string(s: String) -> String {
    format!("{}{}", home_dir().unwrap().to_str().unwrap(), s)
}
