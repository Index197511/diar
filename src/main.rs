extern crate clap;
extern crate sled;

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use diar::command::{Command, to_command};
use diar::types::JumpTo;
use dirs::home_dir;
use sled::Db;
use std::path::Path;

mod add;
mod clear;
mod delete;
mod jump;
mod list;
mod ls;
mod rename;

fn main() {
    let db = Db::open(Path::new(&format!(
        "{}/.diar",
        home_dir().unwrap().as_path().to_string_lossy(),
    )))
    .unwrap();

    let app = App::new("diar")
        .version("2.2.0")
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
        .subcommand(
            SubCommand::with_name("ls")
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
        Some(subcommand_name) => match to_command(subcommand_name) {
            Some(Command::Add) => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    match matches.get_value(subcommand_name, "path") {
                        Some(given_path) => {
                            add::add_favorite(db, key, Some(Path::new(&given_path)))
                        }
                        None => add::add_favorite(db, key, None),
                    }
                }
            }

            Some(Command::Delete) => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    delete::delete_from_db(db, key);
                }
            }

            Some(Command::Rename) => {
                if let Some(old_key) = matches.get_value(subcommand_name, "old_key") {
                    if let Some(new_key) = matches.get_value(subcommand_name, "new_key") {
                        rename::rename_favorite(db, old_key, new_key);
                    }
                }
            }

            Some(Command::List) => list::list_favorites(db),

            Some(Command::Jump) => {
                if let Some(subcommand_matches) = matches.subcommand_matches(subcommand_name) {
                    if subcommand_matches.is_present("project-root") {
                        jump::jump_to(db, JumpTo::ProjectRoot);
                    } else {
                        if let Some(key) = matches.get_value(subcommand_name, "key") {
                            jump::jump_to(db, JumpTo::Key(key));
                        }
                    }
                }
            }

            Some(Command::Clear) => clear::clear_db(db),

            Some(Command::Ls) => {
                if let Some(key) = matches.get_value(subcommand_name, "key") {
                    ls::ls_at_favorite(db, key);
                }
            }

            None => (),
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
