use super::model::Favorite;
use anyhow::Result;
pub trait IRepository {
    fn add(&self, favorite: &Favorite) -> Result<()>;
    fn get_all(&self) -> Result<Vec<Favorite>>;
    fn get(&self, name: &str) -> Result<Option<Favorite>>;
    fn remove(&self, name: &str) -> Result<()>;
    fn remove_all(&self) -> Result<()>;
    fn exists(&self, name: &str) -> Result<bool>;
}
