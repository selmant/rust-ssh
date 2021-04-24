mod commands;
mod io;
mod session;

use std::net::{TcpListener, TcpStream};
use std::thread;
const SERVER_IP: &str = "192.168.0.17:3000";

fn main() {
    let listener = TcpListener::bind(SERVER_IP).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let mut session = session::UserSession::new(stream);
    session.start_session();
}
