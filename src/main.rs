extern crate clap;
extern crate sled;

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use dirs::home_dir;
use std::path::Path;

mod add;
mod clear;
mod delete;
mod jump;
mod list;

fn main() {
    let users_db = format!("{}{}", home_dir().unwrap().to_str().unwrap(), "/.dir");
    let db_path = Path::new(&users_db);
    let app = App::new("Let's bookmark directory you like!")
        .version("1.0.1")
        .author("Index197511 and 4afS")
        .about("A directory favorite tool in Rust.")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a directory you like to favorite")
                .arg(
                    Arg::with_name("path")
                        .help("absolute path")
                        .short("p")
                        .long("path")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("key")
                        .help("key to directory")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a directory from favorite")
                .arg(
                    Arg::with_name("key")
                        .help("named directory")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Display a favorite directory list"))
        .subcommand(
            SubCommand::with_name("jump")
                .about("Jump to your favorite directory")
                .arg(
                    Arg::with_name("key")
                        .help("favorite dirs key")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("clear").about("Clear fav dirs."));

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(subcommand_name) => match subcommand_name {
            "add" => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    match matches.get_value(subcommand_name, "path") {
                        Some(path_to_directory) => {
                            add::add_favorite(Some(Path::new(&path_to_directory)), key, db_path)
                        }
                        None => add::add_favorite(None, key, db_path),
                    }
                }
            }

            "delete" => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    delete::delete_from_db(&key, db_path);
                }
            }

            "list" => list::list_favorites(db_path),

            "jump" => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    jump::jump_if_matched(key, db_path);
                }
            }
            "clear" => clear::clear_db(db_path),
            _ => {
                println!();
            }
        },

        None => {
            println!("Please give args");
            return;
        }
    }
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
