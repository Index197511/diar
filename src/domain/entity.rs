#[macro_use]
use derive_new::new;

#[derive(new)]
pub struct Name<'a>(&'a str);

impl<'a> Name<'a> {
  pub fn name(&self) -> &str {
    self.0
  }
}
