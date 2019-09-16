extern crate clap;
extern crate diar;
extern crate sled;

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use std::path::Path;

use diar::add_favorite;
use diar::delete_favorite;
use diar::jump_dir;
use diar::list_favorite;

fn main() {
    let db_path = Path::new("strage");
    let app = App::new("Let's bookmark directory you like!")
        .version("0.1.0")
        .author("Index197511 and 4afS")
        .about("A directory favorite tool in Rust.")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a directory you like to favorite")
                .arg(
                    Arg::with_name("path")
                        .help("absolute path")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("name")
                        .help("key to directory")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a directory from favorite")
                .arg(
                    // rename "named_directory" "name"
                    Arg::with_name("name")
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
                    // rename "key" "name"
                    Arg::with_name("name")
                    .help("favorite dirs key")
                    .takes_value(true)
                    .required(true)
                    )
            );

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(subcommand_name) => match subcommand_name {
            "add" => {
                if let Some(path_to_directory) = matches.get_path_to_directory(subcommand_name) {
                    if let Some(key) = matches.get_key(subcommand_name) {
                        add_favorite::add_to_db(Path::new(&path_to_directory), key, db_path);
                    }
                }
            }

            "delete" => {
                if let Some(key) = matches.get_key(subcommand_name) {
                    delete_favorite::delete_from_db(&key, db_path);
                }
            }

            "list" => list_favorite::list(db_path),

            "jump" => {
                if let Some(key) = matches.get_key(subcommand_name) {
                    jump_dir::search_and_jump(key, db_path);
                }
            }

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
    fn get_key(&self, subcommand_name: &str) -> Option<String>;
    fn get_path_to_directory(&self, subcommand_name: &str) -> Option<String>;
}

impl GetFromArg for ArgMatches<'_> {
    fn get_key(&self, subcommand_name: &str) -> Option<String> {
        get_value_from_args(self, subcommand_name, "name")
    }

    fn get_path_to_directory(&self, subcommand_name: &str) -> Option<String> {
        get_value_from_args(self, subcommand_name, "path")
    }
}

fn get_value_from_args(
    subcommand: &clap::ArgMatches,
    subcommand_name: &str,
    value_name: &str,
) -> Option<String> {
    subcommand
        .subcommand_matches(subcommand_name.to_string())
        .and_then(|args| args.value_of(value_name.to_string()))
        .map(|name| name.to_string())
}
