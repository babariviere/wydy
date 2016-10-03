use std::io;
use std::io::{BufRead, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Make a connection with the server.
pub fn connect_to_server<A: ToSocketAddrs>(addr: A) -> Result<TcpStream, String> {
    let mut stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(e) => return Err(format!("Can't connect to server: {}", e)),
    };

    if !confirmation_process(&mut stream) {
        return Err("Error in confirmation process".to_string());
    }
    Ok(stream)
}

/// Send and receive confirmation process after connection.
pub fn confirmation_process(stream: &mut TcpStream) -> bool {
    stream.write(b"WYDY").unwrap();
    let mut confirmation = [0; 4];
    stream.read(&mut confirmation).unwrap();
    let confirmation = confirmation.to_vec();
    let confirmation = String::from_utf8(confirmation).unwrap();
    confirmation == "WYDY"
}

/// Send presence to continue communication with the server
pub fn send_presence(stream: &mut TcpStream) {
    stream.write(&[1]).unwrap();
}

/// Send a command to the server
// TODO make this function return Result
pub fn send_command(stream: &mut TcpStream, command: String) {
    // TODO replace unwrap by a match
    send_presence(stream);
    stream.write(command.as_bytes()).unwrap();
    stream.write(b"\n").unwrap();
    // Receive number of options
    command_response(stream);
    // TODO get command response and do something from it
}

fn command_response(stream: &mut TcpStream) {
    let mut response = [0];
    stream.read(&mut response).unwrap();
    match response[0] {
        1 => {
            // Server is executing the command
            receive_running_command(stream);
        }
        2 => {
            // There is multiple command, server needs to receive the choice
            handle_multiple_commands(stream);
        }
        3 => {
            // Used to do output
        }
        _ => {
            // Invalid command
            println!("Please, run a valid command");
            println!("Type 'list commands' to get the list of all commands");
            return;
        }
    }
}

/// Handle multiple commands
fn handle_multiple_commands(stream: &mut TcpStream) {
    let commands = receive_commands(stream);
    for (i, command) in commands.iter().enumerate() {
        println!("[{}] {}", i + 1, command);
    }
    println!("[_] Exit");

    // Read response
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let mut input = String::new();
    lock.read_line(&mut input).unwrap();
    let input = input.trim();
    let choice = match input.parse::<u8>() {
        Ok(c) => c,
        Err(e) => {
            println!("Invalid input, exiting: {}", e);
            u8::max_value()
        }
    };
    stream.write(&[choice]).unwrap();
    let choice = choice as usize;
    if choice >= 1 && choice <= commands.len() {
        receive_running_command(stream);
    } else {
        println!("Exiting...");
    }
}

fn receive_commands(stream: &mut TcpStream) -> Vec<String> {
    let mut num_commands = [0];
    stream.read(&mut num_commands).unwrap();
    let mut commands = Vec::new();
    let mut reader = io::BufReader::new(stream);
    for _ in 0..num_commands[0] {
        let mut read = String::new();
        reader.read_line(&mut read).unwrap();
        let read = read.trim().to_string();
        commands.push(read);
    }
    commands
}

fn receive_running_command(stream: &mut TcpStream) {
    receive_running_command_desc(stream);
    let code = receive_status(stream);
    println!("Command executed with code {}", code);
}

fn receive_running_command_desc(stream: &mut TcpStream) {
    let mut response = String::new();
    let mut reader = io::BufReader::new(stream);
    reader.read_line(&mut response).unwrap();
    print!("{}", response);
}

fn receive_status(stream: &mut TcpStream) -> i64 {
    let mut reader = io::BufReader::new(stream);
    let mut status = String::new();
    reader.read_line(&mut status).unwrap();
    let status = status.trim();
    status.parse().unwrap()
}
