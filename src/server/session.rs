use crate::io::IOOperationHandler;
use std::net::TcpStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{io::prelude::*, thread};
const DEFAULT_PATH: &str = "/home/selmant";
const IDLE_TIMEOUT: Duration = Duration::from_secs(20);

pub(crate) struct UserSession {
    socket: TcpStream,
    wd: PathBuf,
}

impl UserSession {
    pub(crate) fn new(socket: TcpStream) -> UserSession {
        socket.set_read_timeout(Some(IDLE_TIMEOUT)).unwrap();
        let wd = PathBuf::from_str(DEFAULT_PATH).unwrap();
        UserSession { socket, wd}
    }

    pub(crate) fn start_session(&mut self) {
        let mut count = 0;
        loop {
            let mut input = String::new();
            if let Err(e) = self.socket.read_to_string(&mut input) {
                eprintln!("{}", e);
                break;
            };


            println!("{:?} {}", self.socket, count);
            count += 1;
        }
    }
}

impl Drop for UserSession {
    fn drop(&mut self) {
        println!("UserSession dropped.");
    }
}
