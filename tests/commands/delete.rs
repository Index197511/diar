#[cfg(test)]
mod test_delete_from_db {

    use diar::{
        command::CommandError,
        commands::delete::delete_from_db,
        domain::{model::Favorite, repository::IRepository},
    };

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn delete() {
        let fav = Favorite::new("name1", "/");
        let repo = &Repository::new(vec![fav.clone()]);

        let _ = delete_from_db(repo, fav.name()).unwrap();
        let got = repo.get(&fav.name()).unwrap();

        assert!(got.is_none())
    }

    #[test]
    fn not_found_error() {
        let fav = Favorite::new("name1", "/");
        let repo = &Repository::new(Vec::new());

        let result = delete_from_db(repo, fav.name());

        assert_eq!(
            result.err().unwrap().to_string(),
            CommandError::GivenKeyNotFound.to_string()
        )
    }
}
