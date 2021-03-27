use crate::domain::model::Favorite;
use crate::interface::presenter::{error, print_result};
use crate::{command::CommandResult, domain::repository::IRepository};

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
