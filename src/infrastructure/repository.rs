use super::db::DbHandler;
use crate::domain::model::Favorite;
use crate::domain::repository::IRepository;
use anyhow::Result;
use std::io::{Error, ErrorKind};

pub struct Repository(pub DbHandler);

impl IRepository for Repository {
    fn add(&self, favorite: &Favorite) -> Result<Favorite> {
        match self
            .0
             .0
            .insert(favorite.name(), favorite.path().as_bytes().to_vec())
        {
            Ok(_) => Ok(favorite.clone()),
            Err(e) => Err(e.into()),
        }
    }

    fn get_all(&self) -> Result<Vec<Favorite>> {
        let mut favorites: Vec<Favorite> = Vec::new();
        for favorite in self.0 .0.iter() {
            if let Ok((k, v)) = favorite {
                if let (Ok(name), Ok(path)) =
                    (String::from_utf8(k.to_vec()), String::from_utf8(v.to_vec()))
                {
                    favorites.push(Favorite::new(name, path))
                }
            }
        }

        if favorites.is_empty() {
            return Err(
                sled::Error::Io(Error::new(ErrorKind::NotFound, "favorites not found")).into(),
            );
        }

        Ok(favorites)
    }

    fn get(&self, name: &str) -> Result<Option<Favorite>> {
        Ok(match self.0 .0.get(name)? {
            Some(value) => Some(Favorite::new(
                name.to_string(),
                String::from_utf8(value.to_vec())?,
            )),
            None => None,
        })
    }

    fn remove(&self, name: &str) -> Result<Option<Favorite>> {
        let favorite = self.0 .0.get(name)?.and_then(|path| {
            self.0 .0.remove(name).ok()?;
            Some(Favorite::new(
                name.to_string(),
                String::from_utf8(path.to_vec()).ok()?,
            ))
        });

        Ok(favorite)
    }

    fn remove_all(&self) -> Result<()> {
        match self.0 .0.clear() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn exists(&self, name: &str) -> Result<bool> {
        match self.0 .0.get(name)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
