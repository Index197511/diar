extern crate colored;

use crate::domain::model::Command;
use clap::{App, Arg, SubCommand};
use derive_more::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Display)]
pub enum CommandError {
    #[display(fmt = "given key is already exists")]
    GivenKeyIsAlreadyExists,
    #[display(fmt = "invalid path")]
    InvalidPath,
    #[display(fmt = "path not found")]
    PathNotFound,
    #[display(fmt = "git command not found")]
    GitCommandNotFound,
    #[display(fmt = ".git not found")]
    DotGitNotFound,
    #[display(fmt = "given key not found")]
    GivenKeyNotFound,
    #[display(fmt = "skim error occured")]
    SkimErrorOccured,
}

pub enum CommandResult {
    Added(String, String),
    Deleted(String, String),
    Cleared,
    Renamed(String, String),
}

pub fn command<'a, 'b>(command: Command) -> App<'a, 'b> {
    match command {
        Command::Add => SubCommand::with_name("add")
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
        Command::Delete => SubCommand::with_name("delete")
            .about("Delete a directory from favorite")
            .arg(
                Arg::with_name("key")
                    .help("named directory")
                    .takes_value(true)
                    .required(true),
            ),
        Command::Rename => SubCommand::with_name("rename")
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
        Command::List => SubCommand::with_name("list").about("List favorite directories"),
        Command::Jump => SubCommand::with_name("jump")
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
                Arg::with_name("fuzzy-finder")
                    .help("fuzzy finder")
                    .short("f"),
            ),
        Command::Clear => SubCommand::with_name("clear").about("Delete all favorite directories."),
        Command::Ls => SubCommand::with_name("ls")
            .about("ls your favorite directory")
            .arg(
                Arg::with_name("key")
                    .help("favorite dir's key")
                    .takes_value(true)
                    .required(true),
            ),
    }
}
