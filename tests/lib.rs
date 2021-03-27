mod domain {
    mod model;
}

pub(self) mod infrastructure {
    pub mod inmemory {
        pub mod db;
        pub mod repository;
    }

    mod repository;
}

mod commands {
    mod add;
    mod clear;
    mod delete;
    mod jump;
    mod list;
}
