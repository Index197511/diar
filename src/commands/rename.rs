use crate::domain::repository::IRepository;
use crate::{command::CommandError, domain::model::Favorite};

pub fn rename_favorite<T: IRepository>(
    repo: &T,
    old_key: String,
    new_key: String,
) -> anyhow::Result<Favorite> {
    match repo.get(&old_key) {
        Ok(Some(favorite)) => repo
            .remove(&old_key)
            .and_then(|_| repo.add(&Favorite::new(new_key.clone(), favorite.path())))
            .map(|_| favorite),
        _ => Err(CommandError::GivenKeyNotFound.into()),
    }
}
