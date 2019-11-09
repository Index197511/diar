use diar::util::search;
use diar::error::suggest;
use sled::Db;
use std::fs;

pub fn ls_at_favorite(db: Db, key: String) {
    let target = db.get(&key);
    match target {
        Ok(Some(path)) => ls(String::from_utf8(path.to_vec()).unwrap()),
        _ => suggest(&key, search(&key, db)),
    }
}

fn ls(path: String) {
    let mut files: Vec<String> = Vec::new();

    for p in fs::read_dir(&path).unwrap() {
        files.push(
            p.unwrap()
                .path()
                .as_path()
                .to_str()
                .unwrap()
                .replace(&path, ""),
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
