extern crate clap;
extern crate sled;

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use diar::command::{to_command, Command};
use diar::types::{JumpTo, WhereToAdd};
use diar::util::generate_path_string;
use sled::Db;
use std::fs;
use std::path::Path;
use colored::Colorize;

mod add;
mod clear;
mod delete;
mod jump;
mod list;
mod ls;
mod rename;

fn main() {
    let users_db_path = generate_path_string("/.diar".to_owned());
    let db_path = Path::new(&users_db_path);
    rename_diar_directory();
    let db = Db::open(&db_path).unwrap();
    let app = App::new("diar")
        .version("2.3.0")
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
                )
                .arg(
                    Arg::with_name("fzf")
                        .help("fuzzy finder")
                        .short("f")
                    )
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
                            add::add_favorite(db, key, WhereToAdd::Path(Path::new(&given_path)))
                        }
                        None => add::add_favorite(db, key, WhereToAdd::CurrentDirectory),
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
                    } else if subcommand_matches.is_present("key"){
                        if let Some(key) = matches.get_value(subcommand_name, "key") {
                            jump::jump_to(db, JumpTo::Key(key));
                        }
                    } else if subcommand_matches.is_present("fzf"){
                        jump::jump_to(db, JumpTo::Fzf);
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
