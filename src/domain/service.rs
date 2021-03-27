use super::model::Favorite;

pub fn search(searched_word: &str, favorites: Vec<Favorite>) -> Vec<Favorite> {
    favorites
        .into_iter()
        .filter(|favorite| favorite.name().contains(searched_word))
        .collect::<Vec<Favorite>>()
}
