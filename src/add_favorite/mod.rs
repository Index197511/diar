use sled::Db;
use std::path::Path;
use std::env;

pub fn add_to_diar(optional_path: Option<&Path>, key: String, db_path: &Path) -> () {
    let db = Db::open(db_path).unwrap();
    match db.get(&key) {
        Ok(Some(_)) => {
            println!("already exist!");
        }
        _ => {
            match optional_path {
                Some(path) => {
                    if path.exists() {
                        add_path_to_db(path, key, db);
                    } else {
                        println!("This path does not exist!: {}", path.to_str().unwrap());
                    }
                }
                None => {
                    add_current_to_db(key, db);
                }
            }
        }
    }
}

fn add_path_to_db(path: &Path, key: String, db: sled::Db) -> () {
    match db.insert(key, path.to_str().unwrap().as_bytes().to_vec()) {
        Ok(_) => println!("done"),
        Err(e) => println!("{}", e),
    }
}

fn add_current_to_db(key: String, db: sled::Db) -> () {
    match env::current_dir() {
        Ok(current_path) => 
            match db.insert(key, current_path.to_str().unwrap().as_bytes().to_vec()) {
                Ok(_) => println!("done"),
                Err(e) => println!("{}", e),
            },
        Err(e) => println!("{}", e),
    }
}
