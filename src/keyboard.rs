use std::io::{self, Write, BufRead};
use std::sync::{Arc, Mutex};


pub fn handle_keyboard_input(
    tx: crossbeam_channel::Sender<String>, 
    xit_flag: Arc<Mutex<bool>>, stop_flag: Arc<Mutex<bool>>) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    loop {

        print!("Enter a command: ");
        io::stdout().flush().unwrap();
        buffer.clear();
        handle.read_line(&mut buffer).unwrap();
        let input = buffer.trim().to_string();
        if input == "exit" {
            tx.send(input).unwrap();
            break;
        } else if input == "stop" {
            tx.send(input).unwrap();
        } else if input == "start" {
            tx.send(input).unwrap();
        }
    }
}

