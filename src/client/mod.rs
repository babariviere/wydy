pub mod command;
pub mod confirmation;
pub mod run;

use error::Result;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use self::command::*;
use self::confirmation::*;

/// Make a connection with the server.
pub fn connect_to_server<A: ToSocketAddrs>(addr: A) -> Result<TcpStream> {
    let mut stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(_) => {
            error_r!("Server isn't available, run wydyd");
        }
    };

    if !confirmation_process(&mut stream) {
        error_r!("Error in confirmation process");
    }
    Ok(stream)
}

/// Send a command to the server
pub fn send_command(stream: &mut TcpStream, command: &str, locally: bool) -> Result<()> {
    presence_check(stream)?;
    let command = format!("{}\n", command);
    match stream.write(command.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            error_r!("Can't send command: {}", e);
        }
    };
    send_location_flag(stream, locally);
    Ok(())
}

/// Send location flag
pub fn send_location_flag(stream: &mut TcpStream, locally: bool) {
    if locally {
        stream.write(&[2]).unwrap();
    } else {
        stream.write(&[1]).unwrap();
    }
}

/// Receive all the commands from the server
pub fn receive_commands(stream: &mut TcpStream) -> Vec<String> {
    // Receive number of commands
    let mut num_commands = [0];
    stream.read(&mut num_commands).unwrap();

    // Receive all the commands
    let mut commands = Vec::new();
    let mut reader = BufReader::new(stream);
    for _ in 0..num_commands[0] {
        let mut read = String::new();
        reader.read_line(&mut read).unwrap();
        let read = read.trim().to_string();
        commands.push(read);
    }
    commands
}

/// Receive the command location
fn receive_command_location(stream: &mut TcpStream) -> Result<u8> {
    presence_check(stream)?;
    let mut buf = [0];
    stream.read(&mut buf).unwrap();
    println!("Command location {}", buf[0]);
    Ok(buf[0])
}

/// Receive command response.
/// Read response and make a choice based on it.
pub fn command_response(stream: &mut TcpStream) -> Result<()> {
    let mut response = [0];
    stream.read(&mut response).unwrap();
    match response[0] {
        1 => {
            // Server is executing the command
            receive_command_process(stream)?;
        }
        2 => {
            // There is multiple command, server needs to receive the choice
            handle_multiple_commands(stream)?;
        }
        3 => {
            // Used to do output
        }
        _ => {
            // Invalid command
            // Is this reachable?
            error_r!("Please, run a valid command\nType 'list commands' to get the list of all \
                      commands");
        }
    }
    Ok(())
}
