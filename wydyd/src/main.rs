extern crate clap;
extern crate wydyd;

use clap::{Arg, App};
use wydyd::server::initialize_server;

fn main() {
    let app = App::new("wydyd")
        .author("notkild <notkild@gmail.com")
        .about("Wydy daemon to handle all wydy's command")
        .arg(Arg::with_name("log")
            .long("log")
            .takes_value(true)
            .value_name("LOG_LEVEL")
            .help("Set the log level. Default is info (recommanded).\n1: Error, \
                   2: Warn, 3: Info, 4: Debug, 5: Trace, else set it to off."))
        .arg(Arg::with_name("addr")
            .takes_value(true)
            .value_name("ADDRESS")
            .help("Set the server address. Default is 127.0.0.1:9654"))
        .get_matches();

    let mut ip = "127.0.0.1";
    let mut port = 9654;
    let mut log_level = 3;

    if let Some(l) = app.value_of("log") {
        log_level = l.parse::<u8>().unwrap_or(log_level);
    }

    if let Some(a) = app.value_of("addr") {
        let mut split = a.split(':');
        ip = split.next().expect("You didn't specify an IP addr");
        port = match split.next() {
            Some(p) => p.parse::<u16>().unwrap_or(port),
            None => port,
        };
    }

    wydyd::init_logging(log_level);
    initialize_server((ip, port));
}
