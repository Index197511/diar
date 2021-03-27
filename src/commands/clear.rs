use crate::domain::repository::IRepository;

pub fn clear_db<T: IRepository>(repo: T) -> anyhow::Result<()> {
    repo.remove_all()
}
