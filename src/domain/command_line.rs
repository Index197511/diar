use anyhow::Result;

pub trait ICommandLineHandler {
    fn current_path() -> Result<String>;
    fn project_root_path() -> Result<String>;
    fn be_absolute_from(path_str: &str) -> Option<String>;
}
