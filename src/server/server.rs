mod commands;
mod io;
mod session;

use tokio::net::{TcpListener, TcpStream};
const SERVER_IP: &str = "192.168.0.17:3000";

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind(SERVER_IP).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        println!("New connection {:?}", stream);
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut session = session::UserSession::new(stream);
    if let Err(e) = session.start_session().await {
        if e.kind() != tokio::io::ErrorKind::BrokenPipe {
            panic!("{}", e);
        }
    }
}
