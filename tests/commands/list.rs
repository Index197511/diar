#[cfg(test)]
mod test_list_favorites {

    use diar::{commands::list::list_favorites, domain::model::Favorite};

    use crate::infrastructure::inmemory::repository::Repository;

    #[test]
    fn list() {
        let fav1 = Favorite::new("name1", "/");
        let fav2 = Favorite::new("name2", "/");
        let repo = &Repository::new(vec![fav1, fav2]);

        let result = list_favorites(repo).unwrap();

        assert_eq!(result.len(), 2);
    }
}
