pub enum CreateCommand {
    CreateDatabase(String),
}

pub fn execute_create_command(command: &CreateCommand) -> Result<String, String> {
    match command {
        CreateCommand::CreateDatabase(target) => execute_create_database(target)
    }
}

pub fn execute_create_database(target: &String) -> Result<String, String> {
    println!("Create database with target: {}", target);
    Ok(String::from("Success!"))
}

pub fn parse_create_command(args: &[String]) -> Result<CreateCommand, String> {
    if args.len() < 1 {
        return Err(String::from("Target missing on CREATE command"));
    }

    let command_target = args[0].clone().to_uppercase();
    let command_target: &str = &command_target;

    match command_target {
        "DATABASE" => Ok(CreateCommand::CreateDatabase(args[1].clone())),
        _ => Err(format!("Unknown create target: {}", command_target)),
    }
}