extern crate libwydyc;
use libwydyc::client::*;

fn main() {
    let mut server = connect_to_server("127.0.0.1:9654").unwrap();
    send_command(&mut server, "search duck on duckduckgo".to_string());
}
