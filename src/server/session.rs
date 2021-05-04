use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

use crate::{commands::Commands, io::IOOperationHandler};
use std::path::PathBuf;
use std::str::FromStr;
pub const DEFAULT_PATH: &str = "/home/selmant";
const FLAG_BYTE: u8 = 3;

pub(crate) struct UserSession {
    stream: BufStream<TcpStream>,
    io_handler: IOOperationHandler,
}

impl UserSession {
    pub(crate) fn new(socket: TcpStream) -> UserSession {
        let wd = PathBuf::from_str(DEFAULT_PATH).unwrap();
        let io_handler = IOOperationHandler::new(wd);

        let stream = BufStream::new(socket);
        UserSession {
            stream,
            io_handler,
        }
    }

    pub(crate) async fn start_session(&mut self) -> tokio::io::Result<()>  {
        loop {
            let mut buf = Vec::new();
            self.stream.read_until(FLAG_BYTE, &mut buf).await?;
            //Pop FLAG_BYTE from buffer.
            buf.pop();
            let input = String::from_utf8(buf).expect("invalid utf-8");
            self.perform_operations(input.as_str()).await?;
        }
    }
    async fn perform_operations(&mut self, input: &str) -> tokio::io::Result<()> {
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
        self.stream.write_all(&bytes).await?;
        self.stream.flush().await?;
        Ok(())
    }
}

impl Drop for UserSession {
    fn drop(&mut self) {
        println!("{:?} UserSession dropped.", self.stream.get_ref());
    }
}
