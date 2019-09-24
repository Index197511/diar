use std::fmt::{Debug, Display};

pub fn print_done_if_ok<T, E: Display>(result: Result<T, E>) -> () {
    match result {
        Ok(_) => println!("done"),
        Err(e) => println!("{}", e),
    }
}
