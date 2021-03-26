use derive_new::new;
use getset::Getters;

#[derive(new, Getters)]
pub struct Favorite {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    path: String,
}
