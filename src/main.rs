extern crate clap;
extern crate sled;

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use diar::types::JumpTo;
use dirs::home_dir;
use std::path::Path;

mod add;
mod clear;
mod delete;
mod jump;
mod list;
mod rename;
mod ls;

fn main() {
    let users_db = format!("{}{}", home_dir().unwrap().to_str().unwrap(), "/.dir");
    let db_path = Path::new(&users_db);
    let app = App::new("diar")
        .version("2.1.1")
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
        .subcommand(
            SubCommand::with_name("rename")
                .about("Rename favorite directory")
                .arg(
                    Arg::with_name("old_key")
                        .help("old key to favorite directory")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("new_key")
                        .help("new key to favorite directory")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List favorite directories"))
        .subcommand(
            SubCommand::with_name("jump")
                .about("Jump to your favorite directory or root directory of the project")
                .arg(
                    Arg::with_name("key")
                        .help("favorite dirs key")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("project-root")
                        .help("The current project root directory")
                        .long("project-root")
                        .short("p"),
                ),
        )
        .subcommand(SubCommand::with_name("clear").about("Delete all favorite directories."))
        .subcommand(SubCommand::with_name("ls")
                    .about("ls your favorite directory")
                    .arg(
                        Arg::with_name("key")
                            .help("favorite dir's key")
                            .takes_value(true)
                            .required(true),
                        ),
                    );

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

            "rename" => {
                if let Some(old_key) = matches.get_value(subcommand_name, "old_key") {
                    if let Some(new_key) = matches.get_value(subcommand_name, "new_key") {
                        rename::rename_favorite(old_key, new_key, db_path);
                    }
                }
            }

            "list" => list::list_favorites(db_path),

            "jump" => {
                if let Some(subcommand_matches) = matches.subcommand_matches(subcommand_name) {
                    if subcommand_matches.is_present("project-root") {
                        jump::jump_to(JumpTo::ProjectRoot, db_path);
                    } else {
                        if let Some(key) = matches.get_value(subcommand_name, "key") {
                            jump::jump_to(JumpTo::Key(key), db_path);
                        }
                    }
                }
            }
            "clear" => clear::clear_db(db_path),
            "ls" => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    ls::ls_favorite(key, db_path);
                } 
            }
            _ => {
                println!();
            }
        },

        None => {
            println!("diar: try 'diar --help' for more information");
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
