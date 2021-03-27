use crate::{
    command::{print_result, CommandResult},
    domain::repository::IRepository,
};

pub fn clear_db<T: IRepository>(repo: T) {
    print_result(repo.remove_all(), CommandResult::Cleared);
}
