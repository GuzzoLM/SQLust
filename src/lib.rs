pub struct Command {
    pub command: String,
    pub arguments: Vec<String>
}

impl Command {
    pub fn new(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let command = args[1].clone();
        let arguments = args[2..].to_vec();

        Ok(Command { command, arguments })
    }
}