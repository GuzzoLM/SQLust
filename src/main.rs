use std::env;
use std::process;

mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = commands::parse_command(&args[1..]).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = commands::execute_command(&command).unwrap_or_else(|err| {
        println!("Problem executing command. Error: {}", err);
        process::exit(1);
    });

    println!("{}", result);
}
