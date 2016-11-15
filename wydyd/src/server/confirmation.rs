use error::Result;
use std::io::{Read, Write};
use std::net::TcpStream;

/// A process to confirm that the client use wydy.
pub fn confirmation_process(stream: &mut TcpStream) -> Result<()> {
    let addr = stream.peer_addr().unwrap();
    info!("[{}] Receiving confirmation...", addr);
    let mut confirmation = [0; 4];
    stream.read(&mut confirmation).unwrap();
    let confirmation = confirmation.to_vec();
    let confirmation = String::from_utf8(confirmation).unwrap();
    if confirmation == "WYDY" {
        info!("[{}] Confirmation received", addr);
    } else {
        error_r!("[{}] Wrong confirmation: {}", addr, confirmation);
    }
    stream.write(b"WYDY").unwrap();
    Ok(())
}

/// Do a presence check from a wydy user.
pub fn presence_check(stream: &mut TcpStream) -> Result<()> {
    let addr = stream.peer_addr().unwrap();
    let mut presence = [0];
    match stream.read(&mut presence) {
        Ok(_) => {}
        Err(e) => {
            error_r!("Can't receive presence: {}", e);
        }
    };
    if presence[0] != 1 {
        debug!("Receive {} instead of 1", presence[0]);
        error_r!("[{}] Presence check failed.", addr);
    }
    match stream.write(&[1]) {
        Ok(_) => {}
        Err(e) => {
            error_r!("[{}] Can't send presence flag: {}", addr, e);
        }
    }
    Ok(())
}
