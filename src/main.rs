use std::env;
use std::process;

use sqlust::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = Command::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Command: {}, Arguments: {}", command.command, command.arguments[0]);

    println!("{:?}", args);
}
