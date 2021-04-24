use crate::{commands::Commands, io::IOOperationHandler};
use std::net::TcpStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{io::prelude::*, io::BufReader, io::BufWriter};
const DEFAULT_PATH: &str = "/home/selmant";
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const FLAG_BYTE: u8 = 3;

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
        let mut reader = BufReader::new(stream_clone);
        
        let mut buf = Vec::new();

        loop {
            let n = reader.read_until(FLAG_BYTE, &mut buf).expect("asd");
            if n == 0 {
                break;
            }
            //Pop FLAG_BYTE from buffer.
            buf.pop();
            let s = String::from_utf8(buf.clone()).expect("invalid ut-8");
            self.perform_operations(s.as_str());
            buf.clear();
        }
        //
    }
    fn perform_operations(&mut self, input: &str) {
        let stream_clone = self.stream.try_clone().unwrap();
        let mut writer = BufWriter::new(stream_clone);

        let command = Commands::new(input);
        println!("{:?}", command);
        let output = self.io_handler.perform_operation(command).unwrap();
        if let Some(x) = output{
            let mut bytes =x.into_bytes();
            bytes.push(FLAG_BYTE);
            println!("writed");
            writer.write_all(&bytes).expect("error");
            writer.flush().unwrap();
        }
        
        
    }
}

impl Drop for UserSession {
    fn drop(&mut self) {
        println!("UserSession dropped.");
    }
}
