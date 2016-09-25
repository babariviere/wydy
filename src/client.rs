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
// TODO change string to a result of the command
pub fn send_command(stream: &mut TcpStream, command: String) -> String {
    // TODO replace unwrap by a match
    send_presence(stream);
    stream.write(command.as_bytes()).unwrap();
    stream.write(b"\n").unwrap();
    // Receive number of options
    command_response(stream);
    // TODO get command response and do something from it
    // let code = receive_status(stream);
    // println!("Command executed with code {}", code);
    "Everything run smoothly".to_string()
}

fn command_response(stream: &mut TcpStream) {
    let mut response = [0];
    stream.read(&mut response).unwrap();
    let mut reader = io::BufReader::new(stream);
    match response[0] {
        1 => {
            // Server is executing the command
            let mut response = String::new();
            reader.read_line(&mut response).unwrap();
            print!("{}", response);
        }
        2 => {
            // There is multiple command, server needs to receive the choice
        }
        _ => {
            // Invalid command
            println!("Please, run a valid command");
            println!("Type 'list commands' to get the list of all commands");
            return;
        }
    }
}

fn receive_status(stream: &mut TcpStream) -> i64 {
    let mut reader = io::BufReader::new(stream);
    let mut status = String::new();
    reader.read_line(&mut status).unwrap();
    let status = status.trim();
    status.parse().unwrap()
}
