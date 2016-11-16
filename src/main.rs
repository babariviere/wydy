extern crate clap;
extern crate wydy;
extern crate wydyd;

use clap::{App, Arg};
use wydy::client::*;
use wydyd::init_logging;
use wydyd::server::initialize_server;

fn main() {
    let app = App::new("wydy")
        .author("notkild <notkild@gmail.com")
        .about("Wydy client to send command to wydyd server")
        .arg(Arg::with_name("command")
            .takes_value(true)
            .multiple(true)
            .value_name("COMMAND")
            .required_unless_one(&["start-server"])
            .help("Command to send to the server"))
        .arg(Arg::with_name("locally")
            .long("locally")
            .help("Ask the server to execute command locally"))
        .arg(Arg::with_name("start-server")
            .long("start-server")
            .help("Start server instead of launching client"))
        .get_matches();

    if app.is_present("start-server") {
        init_logging(true);
        initialize_server("127.0.0.1:9654");
        return;
    }

    let command = app.values_of("command").unwrap().map(|x| format!("{} ", x)).collect::<String>();
    let command = command.trim();
    let locally = app.is_present("locally");
    let mut server = match connect_to_server("127.0.0.1:9654") {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    match send_command(&mut server, command, locally) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    match command_response(&mut server) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}
