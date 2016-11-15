use command::WLocation;
use error::Result;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use super::confirmation::presence_check;

/// Receive command from wydy client
pub fn receive_command(stream: &mut TcpStream) -> Result<String> {
    let addr = stream.peer_addr().unwrap();
    presence_check(stream)?;
    let mut command = String::new();
    let mut reader = BufReader::new(stream);
    match reader.read_line(&mut command) {
        Ok(_) => {}
        Err(e) => {
            error_r!("[{}] Can't receive command: {}", addr, e);
        }
    }
    let command = command.trim().to_string();
    debug!("[{}] Received command {}", addr, command);
    Ok(command)
}

/// Receive location flag from wydy client
pub fn receive_location_flag(stream: &mut TcpStream) -> Result<WLocation> {
    let addr = stream.peer_addr().unwrap();
    let mut buf = [0];
    match stream.read(&mut buf) {
        Ok(_) => {}
        Err(e) => {
            error_r!("[{}] Can't receive location flag: {}", addr, e);
        }
    }
    debug!("[{}] Received flag {}", addr, buf[0]);
    match buf[0] {
        1 => Ok(WLocation::Both),
        2 => Ok(WLocation::Client),
        i => {
            error_r!("[{}] Unknown location flag {}", addr, i);
        }
    }
}
