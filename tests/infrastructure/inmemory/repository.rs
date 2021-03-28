use super::db::Db;
use derive_more::Display;
use diar::domain::{model::Favorite, repository::IRepository};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Display)]
pub enum InmemoryRepositoryError {
    #[display(fmt = "locked")]
    InternalError,
    #[display(fmt = "already exists")]
    AlreadyExists,
}

pub struct Repository {
    pub db: Db,
}

impl Repository {
    pub fn new(init: Vec<Favorite>) -> Self {
        Repository { db: Db::new(init) }
    }
}

impl IRepository for Repository {
    fn add(
        &self,
        favorite: &diar::domain::model::Favorite,
    ) -> anyhow::Result<diar::domain::model::Favorite> {
        match self.db.handler.lock() {
            Ok(mut db) => {
                let exists = db
                    .iter()
                    .map(|fav| fav.name())
                    .any(|x| x == favorite.name());

                if exists {
                    Err(InmemoryRepositoryError::AlreadyExists.into())
                } else {
                    db.push(favorite.clone());
                    Ok(favorite.clone())
                }
            }
            Err(_) => Err(InmemoryRepositoryError::InternalError.into()),
        }
    }

    fn get_all(&self) -> anyhow::Result<Vec<diar::domain::model::Favorite>> {
        match self.db.handler.lock() {
            Ok(db) => Ok(db.clone()),
            Err(_) => Err(InmemoryRepositoryError::InternalError.into()),
        }
    }

    fn get(&self, name: &str) -> anyhow::Result<Option<diar::domain::model::Favorite>> {
        match self.db.handler.lock() {
            Ok(db) => {
                let filtered = db
                    .clone()
                    .into_iter()
                    .filter(|favorite| -> bool { favorite.name() == name })
                    .collect::<Vec<Favorite>>();

                if filtered.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(filtered.get(0).unwrap().clone()))
                }
            }
            Err(_) => Err(InmemoryRepositoryError::InternalError.into()),
        }
    }

    fn remove(&self, name: &str) -> anyhow::Result<Option<diar::domain::model::Favorite>> {
        match self.db.handler.lock() {
            Ok(mut db) => {
                let got = db
                    .clone()
                    .into_iter()
                    .filter(|favorite| -> bool { favorite.name() == name })
                    .collect::<Vec<Favorite>>();

                if got.is_empty() {
                    Ok(None)
                } else {
                    db.retain(|favorite| favorite.name() != name);
                    Ok(Some(got.get(0).unwrap().clone()))
                }
            }
            Err(_) => Err(InmemoryRepositoryError::InternalError.into()),
        }
    }

    fn remove_all(&self) -> anyhow::Result<()> {
        match self.db.handler.lock() {
            Ok(mut db) => {
                db.clear();
                Ok(())
            }
            Err(_) => Err(InmemoryRepositoryError::InternalError.into()),
        }
    }

    fn exists(&self, name: &str) -> anyhow::Result<bool> {
        match self.db.handler.lock() {
            Ok(db) => Ok(db
                .clone()
                .into_iter()
                .any(|favorite| -> bool { favorite.name() == name })),
            Err(_) => Err(InmemoryRepositoryError::InternalError.into()),
        }
    }
}
