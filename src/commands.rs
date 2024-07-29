#[derive(Debug)]
pub enum Command {
    Hello,
    Exit,
    Stop,
    Start,
}

pub fn parse_command(input: &str) -> Option<Command> {
    match input {
        "hello" => Some(Command::Hello),
        "exit" => Some(Command::Exit),
        "stop" => Some(Command::Stop),
        "start" => Some(Command::Start),
        _ => None,
    }
}

