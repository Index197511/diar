use super::entity;
use derive_getters::Getters;
use derive_new::new;
use std::path::Path;

#[derive(new, Getters)]
pub struct Favorite<'a> {
    name: &'a entity::Name<'a>,
    path: &'a Path,
}
