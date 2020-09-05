use dirs::home_dir;
use std::path::Path;

pub struct Db {
    pub db: sled::Db,
}

impl Db {
    pub fn new(dir: &str) -> Result<Self, sled::Error> {
        match to_abs_path(dir) {
            None => Err(sled::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot found home directory",
            ))),
            Some(path) => sled::Db::open(Path::new(&path)).and_then(|db| Ok(Db { db: db })),
        }
    }
}

pub fn to_abs_path(dir: &str) -> Option<String> {
    home_dir()
        .and_then(|pathbuf| pathbuf.to_str())
        .and_then(|path| format!("{}/{}", path, dir))
}
