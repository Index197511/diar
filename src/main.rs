extern crate clap;
extern crate diar;
extern crate sled;

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use std::path::Path;

use diar::add_favorite;
use diar::delete_favorite;
use diar::list_favorite;
use diar::jump_dir;

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
                        .help("named path")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a directory from favorite")
                .arg(
                    Arg::with_name("name")
                        .help("named path")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Display a favorite directory list"))
        .subcommand(SubCommand::with_name("jump").about("Jump to your favorite directory"));

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(name) => match name {
            "add" => {
                if let Some(path) = matches.get_path(name) {
                    if let Some(name) = matches.get_name(name) {
                        add_favorite::add_to_db(Path::new(&path), name, db_path);
                    }
                }
            }

            "delete" => {
                if let Some(name) = matches.get_name(name) {
                    delete_favorite::delete_from_db(&name, db_path);
                }
            }

            "list" => {
                list_favorite::list(db_path)
            }

            "jump" => {
                if let Some(name) = matches.get_name(name) {
                    jump_dir::search_and_jump(name, db_path);
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
    fn get_name(&self, subcommand: &str) -> Option<String>;
    fn get_path(&self, subcommand: &str) -> Option<String>;
}

impl GetFromArg for ArgMatches<'_> {
    fn get_name(&self, subcommand: &str) -> Option<String> {
        get_arg(self, subcommand, "name")
    }

    fn get_path(&self, subcommand: &str) -> Option<String> {
        get_arg(self, subcommand, "path")
    }
}

fn get_arg(
    subcommand: &clap::ArgMatches,
    subcommand_name: &str,
    value_name: &str,
) -> Option<String> {
    subcommand
        .subcommand_matches(subcommand_name.to_string())
        .and_then(|args| args.value_of(value_name.to_string()))
        .map(|name| name.to_string())
}
