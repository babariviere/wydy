extern crate clap;
extern crate wydyd;

use clap::{Arg, App};
use wydyd::server::initialize_server;

fn main() {
    let app = App::new("wydyd")
        .author("notkild <notkild@gmail.com")
        .about("Wydy daemon to handle all wydy's command")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Enable debug mode"))
        .arg(Arg::with_name("addr")
            .takes_value(true)
            .value_name("ADDRESS")
            .help("Set the server address. Default is 127.0.0.1:9654"))
        .get_matches();

    let mut ip = "127.0.0.1";
    let mut port = 9654;
    let debug = app.is_present("debug");

    if let Some(a) = app.value_of("addr") {
        let mut split = a.split(':');
        ip = split.next().expect("You didn't specify an IP addr");
        port = match split.next() {
            Some(p) => p.parse::<u16>().unwrap_or(port),
            None => port,
        };
    }

    wydyd::init_logging(debug);
    initialize_server((ip, port));
}
