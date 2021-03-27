use crate::domain::model::Favorite;
use crate::{command::CommandError, domain::repository::IRepository};

pub fn delete_from_db<T: IRepository>(repo: T, key: String) -> anyhow::Result<Favorite> {
    match repo.get(&key) {
        Ok(Some(favorite)) => match repo.remove(&key) {
            Ok(_) => Ok(favorite),
            Err(e) => Err(e),
        },
        _ => Err(CommandError::GivenKeyNotFound.into()),
    }
}
