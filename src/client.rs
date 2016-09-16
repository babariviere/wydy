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

/// Send a command to the server
pub fn send_command(stream: &mut TcpStream, command: String) -> String {
    // TODO change string to a result of the command
    // TODO use bufreader
    // TODO replace unwrap by a match
    stream.write(command.as_bytes()).unwrap();
    stream.write(b"\n").unwrap();
    // Receive number of options
    let mut options_num = String::new();
    let mut reader = io::BufReader::new(stream);
    reader.read_line(&mut options_num).unwrap();
    let options_num = options_num.trim().parse::<usize>().unwrap();
    let mut options = String::new();
    for _ in 0..options_num {
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        options.push_str(&response);
        print!("{}", response);
    }
    options
}
