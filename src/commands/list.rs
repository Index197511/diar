use crate::domain::{model::Favorite, repository::IRepository};

pub fn list_favorites<T: IRepository>(repo: &T) -> anyhow::Result<Vec<Favorite>> {
    repo.get_all()
}
