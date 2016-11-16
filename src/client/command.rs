use error::Result;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use super::{receive_command_location, receive_commands};
use super::run::*;

/// Receive command process, description + status code
pub fn receive_command_process(stream: &mut TcpStream) -> Result<()> {
    let location = receive_command_location(stream)?;
    let code = match location {
        1 => run_on_client(stream)?,
        2 => run_on_server(stream)?,
        _ => -1,
    };
    println!("Command executed with code {}", code);
    Ok(())
}

/// Handle multiple commands
pub fn handle_multiple_commands(stream: &mut TcpStream) -> Result<()> {
    let commands = receive_commands(stream);
    for (i, command) in commands.iter().enumerate() {
        println!("[{}] {}", i + 1, command);
    }
    println!("[_] Exit");

    // Read response
    let stdin = ::std::io::stdin();
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
        receive_command_process(stream)?;
    } else {
        println!("Exiting...");
    }
    Ok(())
}
