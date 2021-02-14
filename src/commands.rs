pub mod command_create;

pub enum Command {
    CreateCommand(command_create::CreateCommand),
}

pub fn execute_command(command: &Command) -> Result<String, String> {
    match command {
        Command::CreateCommand(create_command) => {
            command_create::execute_create_command(create_command)
        }
    }
}

pub fn parse_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 3 {
        return Err(String::from("Not enough arguments"));
    }

    let command = args[0].clone().to_uppercase();
    let command: &str = &command;
    let arguments = args[1..].to_vec();

    match command {
        "CREATE" => {
            let create_command_result = command_create::parse_create_command(&arguments);
            match create_command_result {
                Err(err) => Err(err),
                Ok(create_command) => Ok(Command::CreateCommand(create_command)),
            }
        }
        _ => Err(format!("Unknown command: {}", command)),
    }
}
