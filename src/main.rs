extern crate libwydyc;
use libwydyc::client::*;

fn main() {
    let mut server = connect_to_server("127.0.0.1:9654").unwrap();
    let mut args = std::env::args();
    args.next();
    let command = args.map(|x| format!("{} ", x)).collect();
    send_command(&mut server, command);
    command_response(&mut server);
}
