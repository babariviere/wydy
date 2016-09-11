use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn initialize_server() {
    println!("Starting server...");
    let listener = match TcpListener::bind("127.0.0.1:9654") {
        Ok(l) => {
            println!("Server initialized.");
            l
        }
        Err(e) => {
            println!("Error when initializing server: {}", e);
            return;
        }
    };
}
