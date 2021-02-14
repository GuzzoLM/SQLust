pub enum CreateCommand {
    Error(String),
    CreateDatabase(String),
}

pub enum Command {
    CreateCommand(CreateCommand),
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
            let create_command = parse_create_command(&arguments);
            match create_command {
                CreateCommand::Error(err) => return Err(err),
                _ => Ok(Command::CreateCommand(create_command))
            }
        },
        _ => return Err(format!("Unknown command: {}", command))
    }
}

fn parse_create_command(args: &[String]) -> CreateCommand {
    if args.len() < 1 {
        return CreateCommand::Error(String::from("Target missing on CREATE command"));
    }

    let command_target = args[0].clone().to_uppercase();
    let command_target: &str = &command_target;

    match command_target {
        "DATABASE" => CreateCommand::CreateDatabase(args[1].clone()),
        _ => CreateCommand::Error(format!("Unknown create target: {}", command_target)),
    }
}