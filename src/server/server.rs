mod session;
mod io;
mod commands;

use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
const SERVER_IP: &str = "192.168.0.17:3000";

fn main() {
    let listener = TcpListener::bind(SERVER_IP).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);
        thread::spawn(move|| {
            handle_connection(stream);
        });
    }
    let path =Path::new("/home/selmant/Documents/../Downloads");
    for item in path.read_dir().unwrap() {
        println!("{:?}",item.unwrap());
    }
    let string = "mv asd assd -p".to_string();
    let str = &string[..];
    println!("{}", str.len());
}

fn handle_connection(stream: TcpStream) {
    let mut session = session::UserSession::new(stream);
    session.start_session();
}
