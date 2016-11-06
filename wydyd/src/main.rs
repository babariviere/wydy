extern crate wydyd;
use wydyd::server::initialize_server;

fn main() {
    // TODO cli parsing
    let mut args = ::std::env::args();
    args.next();
    wydyd::init_logging(5);
    match args.next() {
        Some(s) => initialize_server(s.as_str()),
        None => initialize_server("127.0.0.1:9654"),
    }
}
