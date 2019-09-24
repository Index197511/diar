use sled::Db;
use std::path::Path;
use std::env;

pub fn add_to_db(path: Option<&Path>, key: String, db_path: &Path) -> () {
    let tree = Db::open(db_path).unwrap();
    match tree.get(&key) {
        Ok(Some(_p)) => {
            println!("already exist!");
        }
        _ => {
            match path {

                Some(p) => {

                    if p.exists() {
                        let _ = tree.insert(key, p.to_str().unwrap().as_bytes().to_vec());
                        println!("done");
                    } else {
                        println!("This path does not exist!: {}", p.to_str().unwrap());
                    }
                    
                }

                None => {
                    let _ = tree.insert(key, env::current_dir().unwrap().to_str().unwrap().as_bytes().to_vec());
                    println!("done");
                }

            }
        }
    }
}
