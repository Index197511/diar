use crate::{
    command::{print_result, CommandResult},
    domain::repository::IRepository,
};
use crate::{domain::model::Favorite, error::error};

pub fn rename_favorite<T: IRepository>(repo: T, old_key: String, new_key: String) {
    match repo.get(&old_key) {
        Ok(Some(favorite)) => {
            print_result(
                repo.remove(&old_key).and_then(|_| {
                    repo.add(&Favorite::new(new_key.clone(), favorite.path().to_string()))
                }),
                CommandResult::Renamed(old_key, new_key),
            );
        }
        _ => {
            error(&format!("This key does not exist: {}", old_key));
        }
    }
}
