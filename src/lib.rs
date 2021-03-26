pub mod command;
pub mod error;
pub mod types;
pub mod util;

pub mod domain {
    pub mod model;
    pub mod repository;
}

pub mod infrastructure {
    pub mod db;
    pub mod repository;
}
