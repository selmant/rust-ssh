mod session;
mod io;
mod commands;

use std::net::{TcpListener, TcpStream};
use std::thread;
const SERVER_IP: &str = "192.168.0.17:3000";

fn main() {
    let listener = TcpListener::bind(SERVER_IP).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
    //let cp = commands::Commands::new("cp asd assd -r");
    let string = "mv asd assd -p".to_string();
    let str = &string[..];
    println!("{}", str.len());
}

fn handle_connection(stream: TcpStream) {
    let mut session = session::UserSession::new(stream);
    session.start_session();
}
