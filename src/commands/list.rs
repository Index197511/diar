use crate::{domain::repository::IRepository, util::print_favorites};

pub fn list_favorites<T: IRepository>(repo: T) {
    print_favorites(repo.get_all().unwrap())
}
