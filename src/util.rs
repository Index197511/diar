extern crate sled;

use crate::domain::{model::Favorite, repository::IRepository};
use dirs::home_dir;

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
