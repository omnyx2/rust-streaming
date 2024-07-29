use std::sync::{Arc, Mutex};
pub fn handle_events(rx: crossbeam_channel::Receiver<String>, exit_flag: Arc<Mutex<bool>>, stop_flag: Arc<Mutex<bool>>) {
    for received in rx {
        match received.as_str() {
            "hello" => println!("Hello to you too!"),
            "exit" => {
                println!("Exiting...");
                *exit_flag.lock().unwrap() = true;
                println!("deadlock open");  
                break;
            }
            "stop" => {
                println!("Stopping input...");
                *stop_flag.lock().unwrap() = true;
            }
            "start" => {
                println!("Starting input...");
                *stop_flag.lock().unwrap() = false;
            }
            _ => println!("Unknown command: {}", received),
        }
    }
}

