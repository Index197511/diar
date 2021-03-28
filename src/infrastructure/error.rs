use derive_more::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Display)]
pub enum InfraError {
    #[display(fmt = "home directory not found")]
    HomeDirectoryNotFound,
    #[display(fmt = "favorites not found")]
    FavoritesNotFound,
}
