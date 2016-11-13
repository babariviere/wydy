use error::Result;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use super::confirmation::receive_presence;

pub fn receive_command(stream: &mut TcpStream) -> Result<String> {
    let addr = stream.peer_addr().unwrap();
    receive_presence(stream)?;
    let mut command = String::new();
    let mut reader = BufReader::new(stream);
    match reader.read_line(&mut command) {
        Ok(_) => {}
        Err(e) => {
            error_r!("[{}] Can't receive command: {}", addr, e);
        }
    }
    let command = command.trim().to_string();
    Ok(command)
}
