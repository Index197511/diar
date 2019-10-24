use sled::Db;
use std::path::Path;

use diar::util::{suggest, search};

pub fn jump_if_matched(user_input: String, db_path: &Path) {
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

fn jump(dest_dir: &Path) {
    println!("{}", dest_dir.to_str().unwrap());
}

