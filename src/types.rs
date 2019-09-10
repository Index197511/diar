#[macro_use]
extern crate failure;

#[derive(Fail)]
pub enum ErrorKind {
    #[fail(display = "illegal argument")]
    IllegalArgument,
    #[fail(display = "path not found")]
    PathNotFound,
}
