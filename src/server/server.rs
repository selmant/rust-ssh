mod commands;
mod io;
mod session;

use tokio::net::{TcpListener, TcpStream};
const SERVER_IP: &str = "192.168.0.17:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(SERVER_IP).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        println!("New connection {:?}", stream);
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut session = session::UserSession::new(stream);
    session.start_session().await.unwrap();
}
