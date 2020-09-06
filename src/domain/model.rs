use super::entity;
use std::path::Path;
use derive_getters::{Getters};
use derive_new::new;


#[derive(new, Getters)]
pub struct Favorite<'a> {
    name: &'a entity::Name<'a>,
    path: &'a Path,
}
