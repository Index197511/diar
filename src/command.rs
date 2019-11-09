extern crate colored;

use colored::Colorize;
use std::fmt::Display;

pub enum Command {
    Add,
    Delete,
    Rename,
    List,
    Jump,
    Clear,
    Ls,
}

pub enum CommandResult {
    Added(String, String),
    Deleted(String, String),
    Cleared,
    Renamed(String, String),
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

pub fn print_result<T, E: Display>(result: Result<T, E>, command_name: CommandResult) {
    match result {
        Ok(_) => match command_name {
            CommandResult::Added(key, path) => {
                println!("{} {} -> {}", "added:".bold().bright_green(), key, path)
            }
            CommandResult::Deleted(key, path) => {
                println!("{} {} -> {}", "deleted:".bold().bright_green(), key, path)
            }
            CommandResult::Cleared => println!("{}", "cleared".bold().bright_green()),
            CommandResult::Renamed(o_key, n_key) => {
                println!("{} {} -> {}", "rename:".bold().bright_green(), o_key, n_key)
            }
        },
        Err(e) => println!("{}", e),
    }
}
