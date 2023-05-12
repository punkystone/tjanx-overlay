use std::{io::Error, time::Duration};

use futures_util::SinkExt;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let try_socket = TcpListener::bind("0.0.0.0:8080").await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);
    loop {
        ws_stream
            .send(Message::Text("Hello World".to_string()))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
