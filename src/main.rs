use std::env;
use std::process;

// use sqlust;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = sqlust::parse_command(&args[1..]).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let sqlust::Command::CreateCommand(create_command) = command {
        match create_command {
            sqlust::CreateCommand::CreateDatabase(target) => println!("Create database with target: {}", target),
            sqlust::CreateCommand::Error(mess) => println!("ERROR: {}", mess),
        }
    }

    println!("{:?}", args);
}
