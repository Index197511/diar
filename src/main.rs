use clap::App;
use clap::ArgMatches;
use colored::Colorize;
use diar::command::CommandResult;
use diar::{
    command::command,
    infrastructure::{db::DbHandler, repository::Repository},
    interface::presenter::print_favorites,
};
use diar::{
    command::{JumpTo, WhereToAdd},
    interface::presenter,
};
use diar::{domain::model::Command, interface::presenter::print_result};
use std::path::Path;
use std::{fs, str::FromStr};

use diar::commands::{add, clear, delete, jump, list, ls, rename};
use dirs::home_dir;

fn main() {
    let users_db_path = generate_path_string("/.diar".to_owned());
    let db_path = Path::new(&users_db_path);
    rename_diar_directory();
    let db = sled::open(&db_path).unwrap();
    let repo = Repository(DbHandler(db));

    let app = App::new("diar")
        .version("2.3.0")
        .author("Index197511 and 4afS")
        .about("A directory favorite tool in Rust.")
        .subcommand(command(Command::Add))
        .subcommand(command(Command::Delete))
        .subcommand(command(Command::Rename))
        .subcommand(command(Command::List))
        .subcommand(command(Command::Jump))
        .subcommand(command(Command::Clear))
        .subcommand(command(Command::Ls));

    let matches = app.get_matches();
    match matches.subcommand_name() {
        Some(subcommand_name) => match Command::from_str(subcommand_name) {
            Ok(Command::Add) => {
                match (
                    matches.get_value(subcommand_name, "key"),
                    matches.get_value(subcommand_name, "path"),
                ) {
                    (Some(key), Some(path)) => {
                        match add::add_favorite(repo, key, WhereToAdd::Path(Path::new(&path))) {
                            Ok(favorite) => print_result(CommandResult::Added(
                                favorite.name().to_string(),
                                favorite.path().to_string(),
                            )),
                            Err(e) => presenter::error(&e.to_string()),
                        }
                    }
                    (Some(key), None) => {
                        match add::add_favorite(repo, key, WhereToAdd::CurrentDirectory) {
                            Ok(favorite) => print_result(CommandResult::Added(
                                favorite.name().to_string(),
                                favorite.path().to_string(),
                            )),
                            Err(e) => presenter::error(&e.to_string()),
                        }
                    }
                    _ => guide_to_help(),
                }
            }

            Ok(Command::Delete) => match matches.get_value(subcommand_name, "key") {
                Some(key) => match delete::delete_from_db(repo, key) {
                    Ok(favorite) => print_result(CommandResult::Deleted(
                        favorite.name().to_string(),
                        favorite.path().to_string(),
                    )),
                    Err(e) => presenter::error(&e.to_string()),
                },
                _ => guide_to_help(),
            },

            Ok(Command::Rename) => {
                match (
                    matches.get_value(subcommand_name, "old_key"),
                    matches.get_value(subcommand_name, "new_key"),
                ) {
                    (Some(old), Some(new)) => match rename::rename_favorite(repo, old, new.clone())
                    {
                        Ok(favorite) => {
                            print_result(CommandResult::Renamed(favorite.name().to_string(), new))
                        }
                        Err(e) => presenter::error(&e.to_string()),
                    },
                    _ => guide_to_help(),
                }
            }

            Ok(Command::List) => match list::list_favorites(repo) {
                Ok(favorites) => print_favorites(favorites),
                Err(e) => presenter::error(&e.to_string()),
            },

            Ok(Command::Jump) => {
                if let Some(subcommand_matches) = matches.subcommand_matches(subcommand_name) {
                    if subcommand_matches.is_present("project-root") {
                        match jump::jump_to(repo, JumpTo::ProjectRoot) {
                            Ok(path) => println!("{}", path),
                            Err(e) => presenter::error(&e.to_string()),
                        };
                    } else if subcommand_matches.is_present("fuzzy-finder") {
                        match jump::jump_to(repo, JumpTo::FuzzyFinder) {
                            Ok(path) => println!("{}", path),
                            Err(e) => presenter::error(&e.to_string()),
                        };
                    } else {
                        match matches.get_value(subcommand_name, "key") {
                            Some(key) => match jump::jump_to(repo, JumpTo::Key(key)) {
                                Ok(path) => println!("{}", path),
                                Err(e) => presenter::error(&e.to_string()),
                            },
                            _ => guide_to_help(),
                        }
                    }
                } else {
                    guide_to_help();
                }
            }

            Ok(Command::Clear) => match clear::clear_db(repo) {
                Ok(_) => print_result(CommandResult::Cleared),
                Err(e) => presenter::error(&e.to_string()),
            },

            Ok(Command::Ls) => match matches.get_value(subcommand_name, "key") {
                Some(key) => match ls::ls_at_favorite(repo, key) {
                    Ok(favorite) => presenter::ls(favorite.path()),
                    Err(e) => presenter::error(&e.to_string()),
                },
                None => guide_to_help(),
            },

            Err(_) => guide_to_help(),
        },

        None => {
            guide_to_help();
        }
    }
}

fn guide_to_help() {
    println!("diar: try 'diar --help' for more information");
}

fn generate_path_string(s: String) -> String {
    format!("{}{}", home_dir().unwrap().to_str().unwrap(), s)
}

trait GetFromArg {
    fn get_value(&self, subcommand_name: &str, value_name: &str) -> Option<String>;
}

impl GetFromArg for ArgMatches<'_> {
    fn get_value(&self, subcommand_name: &str, value_name: &str) -> Option<String> {
        self.subcommand_matches(subcommand_name.to_string())
            .and_then(|args| args.value_of(value_name.to_string()))
            .map(|name| name.to_string())
    }
}

fn rename_diar_directory() {
    let users_db_path = generate_path_string("/.diar".to_owned());
    let changed_db_path = generate_path_string("/.dir".to_owned());
    if !Path::new(&users_db_path).exists() && Path::new(&changed_db_path).exists() {
        println!("This tool requires a .diar directory.");
        println!("Since it was a .dir directory in the previous version,");
        println!("I am trying to replace your .dir directory with a .diar directory.");
        println!("Please type y or n");

        loop {
            let user_ans: String = read();
            match &*user_ans {
                "y" => {
                    let _ = fs::rename(Path::new(&changed_db_path), Path::new(&users_db_path));
                    println!("{} .dir -> .diar", "rename:".bold().bright_green());
                    break;
                }
                "n" => {
                    println!("{} new .diar directory", "generate:".bold().bright_green());
                    break;
                }
                _ => {
                    println!("Please type y or n");
                }
            }
        }
        println!();
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
