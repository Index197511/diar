use crate::{
    domain::{repository::IRepository, service::search},
    error::suggest,
};
use std::fs;

pub fn ls_at_favorite<T: IRepository>(repo: T, key: String) {
    let target = repo.get(&key);
    match target {
        Ok(Some(favorite)) => ls(favorite.path()),
        _ => {
            let favorites = repo.get_all().unwrap();
            suggest(&key, search(&key, favorites))
        }
    }
}

fn ls(path: &str) {
    let mut files: Vec<String> = Vec::new();

    for p in fs::read_dir(path).unwrap() {
        files.push(
            p.unwrap()
                .path()
                .as_path()
                .to_str()
                .unwrap()
                .replace(path, ""),
        );
    }

    let shaped_files = files.iter().fold(String::new(), |join, s| {
        if join == String::new() {
            s.to_string()
        } else {
            join + "  " + s
        }
    });
    println!("{}", shaped_files);
}
