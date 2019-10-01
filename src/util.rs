extern crate sled;

use std::fmt::Display;

use super::types::Favorite;

pub fn print_done_if_ok<T, E: Display>(result: Result<T, E>) -> () {
    match result {
        Ok(_) => println!("done"),
        Err(e) => println!("{}", e),
    }
}

pub fn get_favorites(iter_db: sled::Iter<'_>) -> Vec<Favorite> {
    let mut favorites: Vec<Favorite> = Vec::new();
    let favorites_utf8 = iter_db
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
