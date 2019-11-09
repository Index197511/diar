use diar::util::{search, suggest};
use sled::Db;
use std::fs;

pub fn ls_favorite(db: Db, key: String) {
    let maybe_target = db.get(&key);
    match maybe_target {
        Ok(Some(path)) => {
            let path_string: String = String::from_utf8(path.to_vec()).unwrap();
            ls(path_string);
        }
        _ => {
            suggest(&key, search(&key, db));

        }
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
