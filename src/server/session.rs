use crate::{commands::Commands, io::IOOperationHandler};
use std::net::TcpStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{io::prelude::*, io::BufReader, io::BufWriter};
pub const DEFAULT_PATH: &str = "/home/selmant";
const IDLE_TIMEOUT: Duration = Duration::from_secs(300);
const FLAG_BYTE: u8 = 3;

pub(crate) struct UserSession {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    io_handler: IOOperationHandler,
}

impl UserSession {
    pub(crate) fn new(stream: TcpStream) -> UserSession {
        let wd = PathBuf::from_str(DEFAULT_PATH).unwrap();
        let io_handler = IOOperationHandler::new(wd);

        stream.set_read_timeout(Some(IDLE_TIMEOUT)).unwrap();
        let stream_clone = stream.try_clone().unwrap();
        let reader = BufReader::new(stream);
        let writer = BufWriter::new(stream_clone);
        UserSession {
            reader,
            writer,
            io_handler,
        }
    }

    pub(crate) fn start_session(&mut self) {
        let mut buf = Vec::new();

        loop {
            match self.reader.read_until(FLAG_BYTE, &mut buf) {
                Ok(0) => break,
                Ok(bytes) => bytes,
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        panic!("{}", e);
                    }
                    break;
                }
            };
            //Pop FLAG_BYTE from buffer.
            buf.pop();
            let input = String::from_utf8(buf.clone()).expect("invalid utf-8");
            self.perform_operations(input.as_str());
            buf.clear();
        }
    }
    fn perform_operations(&mut self, input: &str) {
        let wd = self.io_handler.get_wd().to_string_lossy().to_string();
        let command = Commands::new(input, wd.as_str());
        println!("{:?}", command);
        let result = self.io_handler.perform_operation(command.clone());
        let output = match result {
            Ok(value) => value,
            Err(e) => Some(format!("{}: {}", command.as_string(), e)),
        };
        let output = output.unwrap_or_else(|| "ok".to_string());
        let mut bytes = output.into_bytes();
        bytes.push(FLAG_BYTE);
        println!("writed");
        if let Err(e) = self.writer.write_all(&bytes) {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                panic!("{}", e);
            }
        }
        if let Err(e) = self.writer.flush() {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                panic!("{}", e);
            }
        }
    }
}

impl Drop for UserSession {
    fn drop(&mut self) {
        println!("{:?} UserSession dropped.", self.reader.get_ref());
    }
}
