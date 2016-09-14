use config::*;
use std::io;
use std::io::{Read, Write};
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

    thread::spawn(|| {
        println!("Press \'q\' + <Return> to close the server");
        loop {
            let mut stdin = io::stdin();
            let mut recv = [0];
            stdin.read(&mut recv).unwrap();
            let recv = recv[0] as char;
            if recv == 'q' {
                println!("==> Server is closing...");
                ::std::process::exit(0);
            }
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("!!! Client tried to connect: {}", e);
            }
        }
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("==> Client connected {}", addr);
    if !confirmation_process(&mut stream) {
        return;
    }

    println!("==> Client disconnected {}", addr);
}

pub fn confirmation_process(stream: &mut TcpStream) -> bool {
    let addr = stream.peer_addr().unwrap();
    println!("[{}] Receiving confirmation...", addr);
    let mut confirmation = [0; 4];
    stream.read(&mut confirmation).unwrap();
    let confirmation = confirmation.to_vec();
    let confirmation = String::from_utf8(confirmation).unwrap();
    match confirmation == "WYDY" {
        true => println!("[{}] Confirmation received", addr),
        false => {
            println!("[{}] Wrong confirmation: {}", addr, confirmation);
            return false;
        }
    }
    stream.write("WYDY".as_bytes()).unwrap();
    true
}
