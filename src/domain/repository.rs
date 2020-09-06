use super::entity::Name;
use super::model::Favorite;

pub trait IRepository {
  type Error: std::fmt::Display;
  fn add(&self, favorite: &Favorite) -> DbResponse<(), Self::Error>;
  fn get_all(&self) -> DbResponse<Vec<Favorite>, Self::Error>;
  fn get(&self, name: &Name) -> DbResponse<Favorite, Self::Error>;
  fn remove(&self, name: &Name) -> DbResponse<(), Self::Error>;
  fn remove_all(&self) -> DbResponse<(), Self::Error>;
  fn exists(&self, name: &Name) -> DbResponse<bool, Self::Error>;
}

pub type DbResponse<T, E: std::fmt::Display> = Result<T, E>;

pub trait HaveRepository {
  type Repository: IRepository;
  fn provide_repository(&self) -> &Self::Repository;
}
