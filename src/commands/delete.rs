use crate::domain::model::Favorite;
use crate::{command::CommandError, domain::repository::IRepository};

pub fn delete_from_db<T: IRepository>(repo: &T, key: String) -> anyhow::Result<Favorite> {
    match repo.remove(&key)? {
        Some(favorite) => Ok(favorite),
        None => Err(CommandError::GivenKeyNotFound.into()),
    }
}
