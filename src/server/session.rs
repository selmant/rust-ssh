use crate::{commands::Commands, io::IOOperationHandler};
use std::net::TcpStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{io::prelude::*, io::BufReader, io::BufWriter};
const DEFAULT_PATH: &str = "/home/selmant";
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct UserSession {
    stream: TcpStream,
    io_handler: IOOperationHandler,
}

impl UserSession {
    pub(crate) fn new(stream: TcpStream) -> UserSession {
        let wd = PathBuf::from_str(DEFAULT_PATH).unwrap();
        let io_handler = IOOperationHandler::new(wd);

        UserSession { stream, io_handler }
    }

    pub(crate) fn start_session(&mut self) {
        let stream_clone = self.stream.try_clone().unwrap();
        let stream_clone2 = self.stream.try_clone().unwrap();

        let mut reader = BufReader::new(stream_clone);
        let mut writer = BufWriter::new(stream_clone2);
        loop {
            for res in reader.by_ref().lines() {
                if let Ok(line) = res {
                    println!("Received line: {}", line);
                    self.perform_operations(line.as_str());
                }
            }
            //reader.read_until('\u', buf)
        }
        //
    }
    fn perform_operations(&mut self, input: &str) {
        let command = Commands::new(input);
        println!("{:?}", command);
        let output = self.io_handler.perform_operation(command).unwrap();
    }
}

impl Drop for UserSession {
    fn drop(&mut self) {
        println!("UserSession dropped.");
    }
}
