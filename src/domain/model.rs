use super::entity;
use std::path::Path;

pub struct Favorite<'a> {
    pub name: entity::Name<'a>,
    pub path: &'a Path,
}

impl<'a> Favorite<'a> {
    pub fn new(name: &'a str, path: &'a Path) -> Self {
        Favorite {
            name: name,
            path: path,
        }
    }
}
