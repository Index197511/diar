use super::db_handler::Db;
use crate::domain::entity::Name;
use crate::domain::model::Favorite;
use crate::domain::repository::{DbResponse, Repository};
use std::io::{Error, ErrorKind};
use std::path::Path;

impl Repository for Db {
  type Error = sled::Error;

  fn add(&self, favorite: Favorite) -> DbResponse<(), Self::Error> {
    match favorite.path.to_str() {
      None => Err(sled::Error::Io(Error::new(ErrorKind::Other, ""))),
      Some(path) => self.db.insert(favorite.name, path).and_then(|_| Ok(())),
    }
  }

  fn remove(&self, name: Name) -> DbResponse<(), Self::Error> {
    self.db.remove(name).and_then(|_| Ok(()))
  }

  fn remove_all(&self) -> DbResponse<(), Self::Error> {
    self.db.clear().and_then(|_| Ok(()))
  }

  fn get_all(&self) -> DbResponse<Vec<Favorite>, Self::Error> {
    let mut favorites: Vec<Favorite> = Vec::new();
    for favorite in self.db.iter() {
      if let Ok((k, v)) = favorite {
        if let (Ok(name), Ok(path)) = (String::from_utf8(k.to_vec()), String::from_utf8(v.to_vec()))
        {
          favorites.push(Favorite::new(&name, Path::new(&path)))
        }
      }
    }

    if favorites.is_empty() {
      return Err(sled::Error::Io(Error::new(
        ErrorKind::NotFound,
        "favorites not found",
      )));
    }

    Ok(favorites)
  }

  fn get(&self, n: Name) -> DbResponse<Favorite, Self::Error> {
    let name = String::from(n);
    self.db.get(&name).and_then(|e| match e {
      Some(path) => match String::from_utf8(path.to_vec()) {
        Ok(s) => Ok(Favorite::new(&name, Path::new(&s))),
        _ => Err(sled::Error::Io(Error::new(
          ErrorKind::Other,
          "this value cannot convert to string",
        ))),
      },
      None => Err(sled::Error::Io(Error::new(
        ErrorKind::NotFound,
        "this favorite not found",
      ))),
    })
  }

  fn exists(&self, name: Name) -> DbResponse<bool, Self::Error> {
    self.db.get(name).and_then(|e| {
      Ok(match e {
        None => false,
        _ => true,
      })
    })
  }
}
