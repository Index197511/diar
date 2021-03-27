#[cfg(test)]
mod test_add_favorite {

    use diar::{
        commands::clear::clear_db,
        domain::{model::Favorite, repository::IRepository},
    };

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn test_clear_db() {
        let fav = Favorite::new("name1", "/");
        let repo = &Repository::new(vec![fav]);

        let _ = clear_db(repo).unwrap();
        let got = repo.get_all().unwrap();

        assert!(got.is_empty())
    }
}
