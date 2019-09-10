mod diar::types::{ErrorKind};
mod std::path::Path

pub fn generate_path(path_str: String) -> Result<Path, ErrorKind> {
    let path = Path::new(path_str);
    if path.exists() {
        path
    } else {
        PathNotFound
    }
}
