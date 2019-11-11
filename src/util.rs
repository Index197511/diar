extern crate colored;
extern crate sled;

use colored::Colorize;

use std::fmt::Display;

use dirs::home_dir;

use super::types::{CommandName, Favorite};

pub fn print_result<T, E: Display>(result: Result<T, E>, command_name: CommandName) {
    match result {
        Ok(_) => match command_name {
            CommandName::Added((key, path)) => {
                println!("{} {} -> {}", "added:".bold().bright_green(), key, path)
            }
            CommandName::Deleted((key, path)) => {
                println!("{} {} -> {}", "deleted:".bold().bright_green(), key, path)
            }
            CommandName::Cleared => println!("{}", "cleared".bold().bright_green()),
            CommandName::Renamed(o_key, n_key) => {
                println!("{} {} -> {}", "rename:".bold().bright_green(), o_key, n_key)
            }
        },
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

pub fn suggest(input: &str, searched: Vec<Favorite>) {
    println!(
        "{} Key '{}' not found.\n",
        "error:".bold().bright_red(),
        input
    );
    println!("Is this what you are looking for?");
    for (key, path) in searched {
        println!("       {} -> {}", key, path);
    }
}

pub fn search(searched_word: &str, db: sled::Db) -> Vec<Favorite> {
    let iter_db = db.iter();
    let favorites = get_favorites(iter_db);

    favorites
        .into_iter()
        .filter(|(key, _)| key.contains(searched_word))
        .collect::<Vec<Favorite>>()
}

pub fn generate_path_string(s: String) -> String {
    format!("{}{}", home_dir().unwrap().to_str().unwrap(), s)
}
