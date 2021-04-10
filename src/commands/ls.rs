use crate::{
    command::CommandError,
    domain::{model::Favorite, repository::IRepository, service::search},
    interface::presenter::suggest,
};

pub fn ls_at_favorite<T: IRepository>(repo: &T, key: String) -> anyhow::Result<Favorite> {
    let target = repo.get(&key);
    match target {
        Ok(Some(favorite)) => Ok(favorite),
        _ => {
            let favorites = repo.get_all()?;
            suggest(&key, search(&key, favorites));
            Err(CommandError::GivenKeyNotFound.into())
        }
    }
}
