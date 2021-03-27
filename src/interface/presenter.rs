use crate::command::CommandResult;
use crate::domain::model::Favorite;
use colored::Colorize;

pub fn print_favorites(favorites: Vec<Favorite>) {
    for favorite in favorites {
        println!("       {} -> {}", favorite.name(), favorite.path());
    }
}

pub fn error(message: &str) {
    println!("{} {}", "error:".bold().bright_red(), message);
}

pub fn suggest(input: &str, searched: Vec<Favorite>) {
    error(&format!("Key '{}' not found.\n", input));
    println!("Is this what you are looking for?");
    print_favorites(searched)
}

pub fn print_result<T>(result: anyhow::Result<T>, command_result: CommandResult) {
    match result {
        Ok(_) => match command_result {
            CommandResult::Added(key, path) => {
                println!("{} {} -> {}", "added:".bold().bright_green(), key, path)
            }
            CommandResult::Deleted(key, path) => {
                println!("{} {} -> {}", "deleted:".bold().bright_red(), key, path)
            }
            CommandResult::Cleared => println!("{}", "cleared".bold().bright_green()),
            CommandResult::Renamed(o_key, n_key) => {
                println!("{} {} -> {}", "rename:".bold().bright_green(), o_key, n_key)
            }
        },
        Err(e) => println!("{}", e),
    }
}
