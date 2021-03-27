#[cfg(test)]
mod test_ls_at_favorite {

    use diar::{commands::ls::ls_at_favorite, domain::model::Favorite};

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn list() {
        let fav = Favorite::new("name1", "/");
        let repo = &Repository::new(vec![fav.clone()]);

        let result = ls_at_favorite(repo, fav.name()).unwrap();

        assert_eq!(result.path(), fav.path())
    }
}
