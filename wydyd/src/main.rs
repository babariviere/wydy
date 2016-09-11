extern crate libwydyd;
use libwydyd::server::initialize_server;

fn main() {
    initialize_server("127.0.0.1:9654");
}
