use std::string::ToString;

pub struct Command {
    spec: String
}

impl Command {
    pub fn as_bytes(&self) -> &[u8] {
        return self.spec.as_str().as_bytes()
    }

    pub fn auth(password: String) -> Command {
        Command::new(vec!["auth".to_string(), password])
    }

    pub fn select_database(database: u8) -> Command {
        Command::new(vec!["select".to_string(), database.to_string()])
    }

    pub fn set<T: ToString>(key: String, value: T) -> Command {
        Command::new(vec!["set".to_string(), key, value.to_string()])
    }

    pub fn expires<T: ToString>(key: String, expires: u64, value: T) -> Command {
        Command::new(vec!["setex".to_string(), key, expires.to_string(), value.to_string()])
    }

    pub fn get<T: ToString>(key: String) -> Command {
        Command::new(vec!["get".to_string(), key])
    }

    fn new(args: Vec<String>) -> Command {
        return Command{spec: Command::create_command(args)}
    }

    fn create_command(args: Vec<String>) -> String {
        let mut command = String::from("");

        for arg in args.iter() {
            command.push_str(format!("${}\r\n{}\r\n", arg.len(), arg).as_str())
        }

        String::from(format!("*{}\r\n{}", args.len(), command))
    }
}
