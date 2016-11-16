use error::Result;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Send and receive confirmation process after connection.
pub fn confirmation_process(stream: &mut TcpStream) -> bool {
    stream.write(b"WYDY").unwrap();
    let mut confirmation = [0; 4];
    stream.read(&mut confirmation).unwrap();
    let confirmation = confirmation.to_vec();
    let confirmation = String::from_utf8(confirmation).unwrap();
    confirmation == "WYDY"
}

/// Do a presence check to continue communication with the server
pub fn presence_check(stream: &mut TcpStream) -> Result<()> {
    match stream.write(&[1]) {
        Ok(_) => {}
        Err(e) => {
            error_r!("Can't send presence: {}", e);
        }
    };
    let mut buf = [0];
    stream.read(&mut buf).unwrap();
    if buf[0] != 1 {
        error_r!("Invalid presence response {}", buf[0]);
    }
    Ok(())
}
