use super::{db::DbHandler, error::InfraError};
use crate::domain::model::Favorite;
use crate::domain::repository::IRepository;
use anyhow::Result;
use derive_new::new;

#[derive(new)]
pub struct Repository {
    pub db: DbHandler,
}

impl IRepository for Repository {
    fn add(&self, favorite: &Favorite) -> Result<Favorite> {
        match self
            .db
            .handler
            .insert(favorite.name(), favorite.path().as_bytes().to_vec())
        {
            Ok(_) => Ok(favorite.clone()),
            Err(e) => Err(e.into()),
        }
    }

    fn get_all(&self) -> Result<Vec<Favorite>> {
        let mut favorites: Vec<Favorite> = Vec::new();
        for favorite in self.db.handler.iter() {
            if let Ok((k, v)) = favorite {
                let name = String::from_utf8(k.to_vec())?;
                let path = String::from_utf8(v.to_vec())?;

                favorites.push(Favorite::new(name, path))
            }
        }

        if favorites.is_empty() {
            return Err(InfraError::FavoritesNotFound.into());
        }

        Ok(favorites)
    }

    fn get(&self, name: &str) -> Result<Option<Favorite>> {
        Ok(match self.db.handler.get(name)? {
            Some(value) => Some(Favorite::new(
                name.to_string(),
                String::from_utf8(value.to_vec())?,
            )),
            None => None,
        })
    }

    fn remove(&self, name: &str) -> Result<Option<Favorite>> {
        let favorite = self.db.handler.get(name)?.and_then(|path| {
            self.db.handler.remove(name).ok()?;
            Some(Favorite::new(
                name.to_string(),
                String::from_utf8(path.to_vec()).ok()?,
            ))
        });

        Ok(favorite)
    }

    fn remove_all(&self) -> Result<()> {
        match self.db.handler.clear() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn is_exist(&self, name: &str) -> Result<bool> {
        match self.db.handler.get(name)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
