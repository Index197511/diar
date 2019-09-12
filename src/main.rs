extern crate diar;
extern crate clap;
extern crate sled;

use clap::{App, Arg, SubCommand};


fn main() {
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
                        .required(true)
                    )
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a directory from favorite")
                .arg(
                    Arg::with_name("path")
                        .help("absolute path")
                        .takes_value(true)
                        .required(true)
                    )
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Display a favorite directory list")
        )
        .subcommand(
            SubCommand::with_name("jump")
                .about("Jump to your favorite directory")
            );

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(name) =>
            match name {

                "add" => {
                    
                }

                "delete" => {

                }
                
                "list" => {

                }

                "jump" => {

                }

                _ => {
                    println!();
                }

            }

        None => {
            println!("Please give args");
            return;
        }

    }

}
