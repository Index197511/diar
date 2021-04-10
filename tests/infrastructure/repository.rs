use diar::domain::{model::Favorite, repository::IRepository};

use super::inmemory::repository::Repository;

#[test]
pub fn test_add() {
    let repo = Repository::new(Vec::new());

    let fav = Favorite::new("name1", "path1");

    let added = &repo.add(&fav);

    assert_eq!(added.as_ref().unwrap().name(), fav.name());
    assert_eq!(added.as_ref().unwrap().path(), fav.path());
}

#[test]
pub fn test_get_all() {
    let fav1 = Favorite::new("name1", "path1");
    let fav2 = Favorite::new("name2", "path2");
    let repo = Repository::new(vec![fav1.clone(), fav2.clone()]);

    let got = &repo.get_all().unwrap();

    assert_eq!(got[0].name(), fav1.name());
    assert_eq!(got[0].path(), fav1.path());

    assert_eq!(got[1].name(), fav2.name());
    assert_eq!(got[1].path(), fav2.path());
}

#[test]
pub fn test_get() {
    let fav = Favorite::new("name1", "path1");
    let repo = Repository::new(vec![fav.clone()]);

    let got = &repo.get(&fav.name()).unwrap().unwrap();

    assert_eq!(got.name(), fav.name());
    assert_eq!(got.path(), fav.path());
}

#[test]
pub fn test_remove() {
    let fav = Favorite::new("name1", "path1");
    let repo = Repository::new(vec![fav.clone()]);

    let _ = &repo.remove(&fav.name()).unwrap().unwrap();
    let got = &repo.get(&fav.name()).unwrap();

    assert!(got.is_none());
}

#[test]
pub fn test_remove_all() {
    let fav1 = Favorite::new("name1", "path1");
    let fav2 = Favorite::new("name2", "path2");
    let repo = Repository::new(vec![fav1, fav2]);

    let _ = &repo.remove_all();
    let got = &repo.get_all().unwrap();

    assert!(got.is_empty());
}

#[test]
pub fn test_exists() {
    let fav1 = Favorite::new("name1", "path1");
    let repo = Repository::new(vec![fav1.clone()]);

    assert!(repo.is_exist(&fav1.name()).unwrap());
}
