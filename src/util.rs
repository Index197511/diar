use std::path::Path;
use super::types::ErrorKind;

pub fn generate_path(path_str: &str) -> Result<&Path, ErrorKind> {
    let path = Path::new(path_str);
    if path.exists() {
        Ok(path)
    } else {
        Err(ErrorKind::PathNotFound)
    }
}
