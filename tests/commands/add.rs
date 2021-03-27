#[cfg(test)]
mod test_add_favorite {
    use std::path::Path;

    use diar::{
        command::CommandError,
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

        assert_eq!(
            result.err().unwrap().to_string(),
            CommandError::GivenKeyIsAlreadyExists.to_string()
        );
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

    #[test]
    fn test_add_favorite_with_invalid_path() {
        let path = Path::new("invalid path");
        let fav = Favorite::new("name1", path.to_str().unwrap());

        let repo = Repository::new(Vec::new());

        let result = add_favorite(&repo, fav.name(), WhereToAdd::Path(&path));

        assert_eq!(
            result.err().unwrap().to_string(),
            CommandError::PathNotFound.to_string()
        );
    }

    //TODO: add tests for add_favorite given WhereToAdd::CurrentDirectory
}
