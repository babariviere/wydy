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

    // Handle input to close the server
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

    // Handle all new connections
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
        // Receive command
        let mut presence = [0];
        match stream.read(&mut presence) {
            Ok(_) => {}
            Err(e) => {
                println!("!!! Can't receive presence: {}", e);
                break;
            }
        };
        if presence[0] != 1 {
            break;
        }
        let mut command = String::new();
        {
            let mut reader = io::BufReader::new(&mut stream);
            match reader.read_line(&mut command) {
                Ok(_) => {}
                Err(e) => {
                    println!("!!! Can't receive command: {}", e);
                    break;
                }
            };
        }
        let command = command;
        // Only for verbose
        // print!("[{}] {}", addr, command);
        let commands = parse_command(command);

        // TODO add option to send output
        let action = send_command_response(&mut stream, &commands);
        let mut command = WCommand::new("", "");
        match action {
            1 => {
                command = commands[0].clone();
            }
            2 => {
                // TODO send choices
                command = match handle_multiple_commands(&mut stream, commands) {
                    Some(c) => c,
                    None => break,
                };
            }
            0 => break,
            _ => {}
        }
        let send = format!("executing \"{}\"\n", command.command());
        stream.write(send.as_bytes()).unwrap();
        println!("[{}] {}", addr, command.desc());
        let code = command.run();
        stream.write(format!("{}\n", code).as_bytes()).unwrap();
    }

    println!("==> Client disconnected {}", addr);
}

/// Handle multiple commands
fn handle_multiple_commands(stream: &mut TcpStream, commands: Vec<WCommand>) -> Option<WCommand> {
    // Send choices
    let num_commands = [commands.len() as u8];
    match stream.write(&num_commands) {
        Ok(_) => {}
        Err(e) => {
            println!("!!! Can't send number of command: {}", e);
            return None;
        }
    };
    for command in &commands {
        stream.write(format!("{}\n", command.desc()).as_bytes()).unwrap();
    }
    let mut response = [0];
    stream.read(&mut response).unwrap_or(0);
    let response = response[0] as usize;
    if response >= 1 && response <= commands.len() {
        Some(commands[response - 1].clone())
    } else {
        println!("Choose to exit");
        None
    }
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
