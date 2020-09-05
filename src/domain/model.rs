use super::entity;
use std::path::Path;

pub struct Favorite<'a> {
    pub name: entity::Name<'a>,
    pub path: &'a Path,
}
