extern crate clap;
extern crate wydy;

use clap::{App, Arg};
use wydy::client::*;

fn main() {
    let app = App::new("wydy")
        .author("notkild <notkild@gmail.com")
        .about("Wydy client to send command to wydyd server")
        .arg(Arg::with_name("command")
            .takes_value(true)
            .value_name("COMMAND")
            .required(true)
            .help("Command to send to the server"))
        .arg(Arg::with_name("locally")
            .long("locally")
            .help("Ask the server to execute command locally"))
        .get_matches();

    let command = app.value_of("command").unwrap();
    let locally = app.is_present("locally");
    let mut server = match connect_to_server("127.0.0.1:9654") {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    send_command(&mut server, command);
    command_response(&mut server);
}
