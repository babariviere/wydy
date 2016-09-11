extern crate libwydyc;
use libwydyc::client::connect_to_server;

fn main() {
    connect_to_server("127.0.0.1:9654").unwrap();
}
