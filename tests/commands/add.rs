#[cfg(test)]
mod test_add_favorite {
    use std::path::Path;

    use diar::{
        commands::add::{add_favorite, WhereToAdd},
        domain::model::Favorite,
    };

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn test_add_favorite_if_exists() {
        let path = Path::new("/");
        let fav = Favorite::new("name1", path.to_str().unwrap());

        let repo = Repository::new(vec![fav.clone()]);

        let result = add_favorite(&repo, fav.name(), WhereToAdd::Path(&path));

        assert!(result.is_err());
    }

    #[test]
    fn test_add_favorite_with_path() {
        let path = Path::new("/");
        let fav = Favorite::new("name1", path.to_str().unwrap());

        let repo = Repository::new(Vec::new());

        let result = add_favorite(&repo, fav.name(), WhereToAdd::Path(&path)).unwrap();

        assert_eq!(result.name(), fav.name());
        assert_eq!(result.path(), fav.path());
    }

    //TODO: add tests for add_favorite given WhereToAdd::CurrentDirectory
}
