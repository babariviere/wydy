use error::Result;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use super::confirmation::presence_check;
use wydyd::command::{WCommand, WLocation};

/// Run the command when the location is Client
pub fn run_on_client(stream: &mut TcpStream) -> Result<i32> {
    let mut reader = BufReader::new(stream);
    let mut command = String::new();
    reader.read_line(&mut command).unwrap();
    let mut desc = String::new();
    reader.read_line(&mut desc).unwrap();
    println!("{}", desc);
    let command = WCommand::new(command, desc, WLocation::Client);
    Ok(command.run())
}

/// When command is run on server, receive status
pub fn run_on_server(stream: &mut TcpStream) -> Result<i32> {
    let cmd_desc = receive_running_command_desc(stream);
    print!("{}", cmd_desc);
    presence_check(stream)?;
    receive_status(stream)
}

/// Receive the status code of the running command
fn receive_status(stream: &mut TcpStream) -> Result<i32> {
    let mut reader = BufReader::new(stream);
    let mut status = String::new();
    match reader.read_line(&mut status) {
        Ok(_) => {}
        Err(e) => {
            error_r!("Error when receiving status: {}", e);
        }
    };
    let status = status.trim();
    match status.parse() {
        Ok(i) => Ok(i),
        Err(e) => {
            error_r!("Error when parsing status code: {}", e);
        }
    }
}

/// Receive description of the running command
fn receive_running_command_desc(stream: &mut TcpStream) -> String {
    let mut response = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut response).unwrap();
    response
}
