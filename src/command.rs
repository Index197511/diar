pub enum Command {
    Add,
    Delete,
    Rename,
    List,
    Jump,
    Clear,
    Ls
}

pub fn to_command(command: &str) -> Option<Command> {
    match command {
        "add" => Some(Command::Add),
        "delete" => Some(Command::Delete),
        "rename" => Some(Command::Rename),
        "list" => Some(Command::List),
        "jump" => Some(Command::Jump),
        "clear" => Some(Command::Clear),
        "ls" => Some(Command::Ls),
        _ => None,
    }
}
