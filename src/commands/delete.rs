use crate::command::CommandResult;
use crate::domain::repository::IRepository;
use crate::interface::presenter::{error, print_result};

pub fn delete_from_db<T: IRepository>(repo: T, key: String) {
    match repo.get(&key) {
        Ok(Some(favorite)) => {
            print_result(
                repo.remove(&key),
                CommandResult::Deleted(key, favorite.path().clone()),
            );
        }
        _ => error(&format!("This key does not exist!: {}", key)),
    };
}
