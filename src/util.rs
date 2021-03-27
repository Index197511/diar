extern crate sled;

use crate::domain::{model::Favorite, repository::IRepository};
use dirs::home_dir;
use sled::Db;

fn from_utf8s(favorite_ivec: (sled::IVec, sled::IVec)) -> Option<Favorite> {
    let key_utf8 = favorite_ivec.0.to_vec();
    let path_utf8 = favorite_ivec.1.to_vec();

    match (String::from_utf8(key_utf8), String::from_utf8(path_utf8)) {
        (Ok(name), Ok(path)) => Some(Favorite::new(name, path)),
        _ => None,
    }
}

pub fn print_favorites(favorites: Vec<Favorite>) {
    for favorite in favorites {
        println!("       {} -> {}", favorite.name(), favorite.path());
    }
}

pub fn search<T: IRepository>(searched_word: &str, repo: T) -> Vec<Favorite> {
    let favorites = repo.get_all().unwrap();

    favorites
        .into_iter()
        .filter(|favorite| favorite.name().contains(searched_word))
        .collect::<Vec<Favorite>>()
}

pub fn generate_path_string(s: String) -> String {
    format!("{}{}", home_dir().unwrap().to_str().unwrap(), s)
}
