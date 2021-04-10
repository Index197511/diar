use std::sync::{Arc, Mutex};

use diar::domain::model::Favorite;

#[derive(Clone)]
pub struct Db {
    pub handler: Arc<Mutex<Vec<Favorite>>>,
}

impl Db {
    pub fn new(init: Vec<Favorite>) -> Self {
        Db {
            handler: Arc::new(Mutex::new(init)),
        }
    }
}
