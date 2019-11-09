extern crate colored;

use colored::Colorize;
use super::types::Favorite;
use super::util::print_favorites;

pub enum GetProjectRootError {
    GitCommandNotFound,
    DotGitNotFound,
}

pub fn print_error(message: &str) {
    println!("{} {}", "error:".bold().bright_red(), message)
}

pub fn suggest(input: &str, searched: Vec<Favorite>) {
    print_error(&format!("Key '{}' not found.\n", input));
    println!("Is this what you are looking for?");
    print_favorites(searched)
}

