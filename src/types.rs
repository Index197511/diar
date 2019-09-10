#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "illegal argument")]
    IllegalArgument,
    #[fail(display = "path not found")]
    PathNotFound,
}
