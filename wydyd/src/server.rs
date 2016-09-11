use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

pub fn initialize_server<A: ToSocketAddrs>(addr: A) {
    println!("==> Starting server...");
    let listener = match TcpListener::bind(addr) {
        Ok(l) => {
            println!("==> Server initialized with address {}.",
                     l.local_addr().unwrap());
            l
        }
        Err(e) => {
            println!("!!! Error when initializing server: {}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {}
            Err(e) => {
                println!("Client tried to connect: {}", e);
            }
        }
    }
}
