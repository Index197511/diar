use crate::util::search;
use crate::{domain::repository::IRepository, error::suggest};
use std::fs;

pub fn ls_at_favorite<T: IRepository>(repo: T, key: String) {
    let target = repo.get(&key);
    match target {
        Ok(Some(favorite)) => ls(favorite.path()),
        _ => suggest(&key, search(&key, repo)),
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
