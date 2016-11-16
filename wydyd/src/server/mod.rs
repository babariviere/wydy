pub mod confirmation;
pub mod io;

use command::*;
use env::Vars;
use error::Result;
use self::confirmation::*;
use self::io::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;

/// Initialize server with address
pub fn initialize_server<A: ToSocketAddrs>(addr: A) {
    info!("Starting server...");
    let listener = match TcpListener::bind(addr) {
        Ok(l) => {
            info!("Server initialized with address {}.",
                  l.local_addr().unwrap());
            l
        }
        Err(e) => {
            error!("Cannot initialize server: {}", e);
            return;
        }
    };
    info!("Press <Ctrl+C> to close the server");

    let vars = Arc::new(Mutex::new(Vars::load()));

    // Handle all new connections
    for stream in listener.incoming() {
        let vars = vars.clone();
        match stream {
            Ok(stream) => {
                thread::Builder::new()
                    .name(format!("{}", stream.peer_addr().unwrap()))
                    .spawn(move || {
                        handle_client(stream, vars).unwrap();
                    })
                    .unwrap();
            }
            Err(e) => {
                error!("Client tried to connect {}", e);
            }
        }
    }

}

/// Handle client and do stuff with him.
pub fn handle_client(mut stream: TcpStream, vars: Arc<Mutex<Vars>>) -> Result<()> {
    let addr = stream.peer_addr().unwrap();
    info!("[{}] Client connected", addr);
    confirmation_process(&mut stream)?;

    // We keep the loop because later we will use autocompletion
    loop {
        // TODO remove unwrap
        // Receive command
        let command = receive_command(&mut stream)?;
        let prefered_location = receive_location_flag(&mut stream)?;
        let commands = parse_user_command(command, &vars);

        // TODO add option to send output
        let action = send_command_response(&mut stream, &commands);
        let command = match action {
            1 => commands[0].clone(),
            2 => {
                match handle_multiple_commands(&mut stream, commands) {
                    Some(c) => c,
                    None => break,
                }
            }
            _ => break,
        };
        // Here with send the command if it's the client that are going to run it or on the server
        let location = send_command_location(&mut stream, &command, &prefered_location)?;
        debug!("[{}] Command location {:?}", addr, location);
        match location {
            WLocation::Client => {
                let send = format!("{}\n{}\n", command.command(), command.desc());
                stream.write(send.as_bytes()).unwrap();
            }
            WLocation::Server => {
                let send = format!("executing \"{}\"\n", command.command());
                stream.write(send.as_bytes()).unwrap();
                debug!("[{}] {}\n>>> {}", addr, command.desc(), command.command());
                let code = command.run();
                debug!("[{}] command `{}` exited with error code {}",
                       addr,
                       command.command(),
                       code);
                presence_check(&mut stream)?;
                let send = format!("{}\n", code);
                stream.write(send.as_bytes()).unwrap();
                // Temp because see at the start of loop
            }
            _ => break,
        }
        break;
    }

    info!("[{}] Client disconnected", addr);
    Ok(())
}

/// Handle multiple commands
fn handle_multiple_commands(stream: &mut TcpStream, commands: Vec<WCommand>) -> Option<WCommand> {
    // Send choices
    let num_commands = [commands.len() as u8];
    match stream.write(&num_commands) {
        Ok(_) => {}
        Err(e) => {
            error!("Can't send number of command: {}", e);
            return None;
        }
    };
    for command in &commands {
        stream.write(format!("{}\n", command.desc()).as_bytes()).unwrap();
    }
    let mut response = [0];
    stream.read(&mut response).unwrap_or(0);
    let response = response[0] as usize;
    if response >= 1 && response <= commands.len() {
        Some(commands[response - 1].clone())
    } else {
        let addr = stream.peer_addr().unwrap();
        info!("[{}] Choose to exit", addr);
        debug!("[{}] Response: {}", addr, response);
        None
    }
}

/// Send response to the command that the server receive
/// Return the response value
fn send_command_response(stream: &mut TcpStream, commands: &[WCommand]) -> u8 {
    match commands.len() {
        1 => {
            stream.write(&[1]).unwrap();
            1
        }
        n if n > 1 => {
            stream.write(&[2]).unwrap();
            2
        }
        _ => {
            stream.write(&[u8::max_value()]).unwrap();
            0
        }
    }
}

/// Send the location where the command would be run.
/// Return error if prefered location doesn't match command possible location.
fn send_command_location(stream: &mut TcpStream,
                         command: &WCommand,
                         prefered_location: &WLocation)
                         -> Result<WLocation> {
    let addr = stream.peer_addr().unwrap();
    presence_check(stream)?;
    if !prefered_location.is_compatible(command.location()) {
        stream.write(&[3]).unwrap();
        error_r!("[{}] Command can't be run on client boxe.", addr);
    }
    if *prefered_location == WLocation::Client {
        stream.write(&[1]).unwrap();
        Ok(WLocation::Client)
    } else {
        match *command.location() {
            WLocation::Server | WLocation::Both => {
                stream.write(&[2]).unwrap();
                Ok(WLocation::Server)
            }
            WLocation::Client => {
                stream.write(&[1]).unwrap();
                Ok(WLocation::Client)
            }
        }
    }
}
