use dirs::home_dir;
use std::path::Path;

pub struct DbHandler(pub sled::Db);

impl DbHandler {
    pub fn new(dir: &str) -> Result<Self, sled::Error> {
        match to_abs_path(dir) {
            None => Err(sled::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot found home directory",
            ))),
            Some(path) => sled::open(Path::new(&path)).map(DbHandler),
        }
    }
}

fn to_abs_path(dir: &str) -> Option<String> {
    Some(format!("{}/{}", home_dir()?.to_str()?, dir))
}
