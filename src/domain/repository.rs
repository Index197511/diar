use super::model::Favorite;
use super::entity::Name;

pub trait Repository {
  type Error;
  fn add(&self, favorite: Favorite) -> DbResponse<(), Self::Error>;
  fn remove(&self, name: Name) -> DbResponse<(), Self::Error>;
  fn get_all(&self) -> DbResponse<Vec<Favorite>, Self::Error>;
  fn get(&self, name: Name) -> DbResponse<Favorite, Self::Error>;
  fn exist(&self, name: Name) -> DbResponse<bool, Self::Error>;
}

pub type DbResponse<T, E: std::fmt::Display> = Result<T, E> ;