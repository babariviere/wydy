use command::*;
use std::io;
use std::io::{BufRead, Read, Write};
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

    loop {
        // TODO remove unwrap
        // TODO use bufreader
        // Receive command
        let mut presence = [0];
        stream.read(&mut presence).unwrap();
        if presence[0] != 1 {
            break;
        }
        let mut command = String::new();
        {
            let mut reader = io::BufReader::new(&mut stream);
            reader.read_line(&mut command).unwrap();
        }
        let command = command;
        // Only for verbose
        print!("[{}] {}", addr, command);
        let commands = parse_command(command);
        for command in &commands {
            println!("[{}] {}", addr, command.desc());
        }

        let action = send_command_response(&mut stream, &commands);
        match action {
            1 => {
                let send = format!("executing \"{}\"\n", commands[0].command());
                stream.write(send.as_bytes()).unwrap();
            }
            2 => {
                // TODO send choices
            }
            0 => break,
            _ => {}
        }
    }

    println!("==> Client disconnected {}", addr);
}

/// Send response to the command that the server receive
/// If the return value then
fn send_command_response(stream: &mut TcpStream, commands: &Vec<WCommand>) -> u8 {
    match commands.len() {
        1 => {
            stream.write(&[1]).unwrap();
            1
        }
        n if n > 1 => {
            stream.write(&[2]).unwrap();
            2
        }
        _ => {
            stream.write(&[u8::max_value()]).unwrap();
            0
        }
    }
}



pub fn confirmation_process(stream: &mut TcpStream) -> bool {
    let addr = stream.peer_addr().unwrap();
    println!("[{}] Receiving confirmation...", addr);
    let mut confirmation = [0; 4];
    stream.read(&mut confirmation).unwrap();
    let confirmation = confirmation.to_vec();
    let confirmation = String::from_utf8(confirmation).unwrap();
    if confirmation == "WYDY" {
        println!("[{}] Confirmation received", addr);
    } else {
        println!("[{}] Wrong confirmation: {}", addr, confirmation);
        return false;
    }
    stream.write(b"WYDY").unwrap();
    true
}
