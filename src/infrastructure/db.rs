use dirs::home_dir;
use sled::Error::Io;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct DbHandler {
    pub handler: sled::Db,
}

impl DbHandler {
    pub fn new(directory_name: &str) -> Result<Self, sled::Error> {
        match to_abs_path(directory_name) {
            None => Err(Io(Error::new(
                ErrorKind::NotFound,
                "cannot found home directory",
            ))),
            Some(path) => sled::open(Path::new(&path)).map(|handler| DbHandler { handler }),
        }
    }
}

fn to_abs_path(dir: &str) -> Option<String> {
    Some(format!("{}/{}", home_dir()?.to_str()?, dir))
}
