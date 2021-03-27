use std::sync::{Arc, Mutex};

use diar::domain::model::Favorite;

#[derive(Clone)]
pub struct Db(pub Arc<Mutex<Vec<Favorite>>>);

impl Db {
    pub fn new(init: Vec<Favorite>) -> Self {
        Db(Arc::new(Mutex::new(init)))
    }
}
