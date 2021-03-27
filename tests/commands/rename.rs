#[cfg(test)]
mod test_rename_favorite {
    use diar::{
        command::CommandError,
        commands::rename::rename_favorite,
        domain::{model::Favorite, repository::IRepository},
    };

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn rename() {
        let fav = Favorite::new("name1", "path1");
        let new = "name2".to_string();

        let repo = Repository::new(vec![fav.clone()]);

        let _ = rename_favorite(&repo, fav.name(), new.clone());
        let got = repo.get(&new).unwrap().unwrap();

        assert_eq!(got.name(), new);
        assert_eq!(got.path(), fav.path());
    }

    #[test]
    fn not_found() {
        let fav = Favorite::new("name1", "path1");
        let name = "name2".to_string();

        let repo = Repository::new(vec![fav]);

        let result = rename_favorite(&repo, name, "new".to_string());

        assert_eq!(
            result.err().unwrap().to_string(),
            CommandError::GivenKeyNotFound.to_string()
        );
    }
}
