#[cfg(test)]
mod test_jump_to {

    use diar::{
        command::CommandError,
        commands::jump::{jump_to, JumpTo},
        domain::model::Favorite,
    };

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn with_key() {
        let fav = Favorite::new("name1", "/");
        let repo = &Repository::new(vec![fav.clone()]);

        let result = jump_to(repo, JumpTo::Key(fav.name())).unwrap();

        assert_eq!(result, "/")
    }

    #[test]
    fn with_key_but_not_exists() {
        let repo = &Repository::new(Vec::new());

        let result = jump_to(repo, JumpTo::Key("".to_string()));

        assert_eq!(
            result.err().unwrap().to_string(),
            CommandError::GivenKeyNotFound.to_string()
        )
    }

    //TODO: add tests for jump_to given JumpTo::FuzzyFinder and JumpTo::ProjectRoot
}
