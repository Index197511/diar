use crate::interface::presenter::print_result;
use crate::{command::CommandResult, domain::repository::IRepository};

pub fn clear_db<T: IRepository>(repo: T) {
    print_result(repo.remove_all(), CommandResult::Cleared);
}
