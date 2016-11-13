pub mod confirmation;
pub mod io;

use command::*;
use env::Vars;
use error::Result;
use self::confirmation::*;
use self::io::*;
use std::io::{Read, Write, stdin};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;

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

    // Handle input to close the server
    thread::spawn(|| {
        info!("Press \'q\' + <Return> to close the server");
        handle_exit();
    });

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

/// Handle user input to close server
pub fn handle_exit() {
    loop {
        let mut stdin = stdin();
        let mut recv = [0];
        stdin.read(&mut recv).unwrap();
        let recv = ::std::char::from_u32(recv[0] as u32).unwrap();
        if recv == 'q' {
            info!("Server is now closed");
            ::std::process::exit(0);
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
        debug!("[{}] Raw command {}", addr, command);
        let commands = parse_user_command(command, &vars);

        // TODO add option to send output
        let action = send_command_response(&mut stream, &commands);
        let mut command = WCommand::new("", "", WLocation::Null);
        match action {
            1 => {
                command = commands[0].clone();
            }
            2 => {
                command = match handle_multiple_commands(&mut stream, commands) {
                    Some(c) => c,
                    None => break,
                };
            }
            0 => break,
            _ => {}
        }
        let send = format!("executing \"{}\"\n", command.command());
        stream.write(send.as_bytes()).unwrap();
        debug!("[{}] {}\n>>> {}", addr, command.desc(), command.command());
        let code = command.run();
        debug!("[{}] command `{}` exited with error code {}",
               addr,
               command.command(),
               code);
        receive_presence(&mut stream)?;
        let send = format!("{}\n", code);
        stream.write(send.as_bytes()).unwrap();
        info!("[{}] Client disconnected", addr);
        return Ok(());
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
        info!("[{}] Choose to exit", stream.peer_addr().unwrap());
        debug!("response: {}", response);
        None
    }
}

/// Send response to the command that the server receive
/// If the return value then
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
