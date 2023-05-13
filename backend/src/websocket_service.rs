use futures_util::SinkExt;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::Receiver,
};
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

use crate::{
    errors::{bind_error::BindError, websocket_connection_error::WebSocketConnectionError},
    tjanx_service::TjanXService,
};
#[allow(clippy::missing_errors_doc)]
pub async fn start_websocket(
    rx: Receiver<String>,
    tjanx_service: TjanXService,
    port: u16,
) -> Result<(), BindError> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("{}", format!("Listening on Port {port}"));
    while let Ok((stream, _)) = listener.accept().await {
        let rx = rx.resubscribe();
        let tjanx_service = tjanx_service.clone();
        tokio::spawn(async {
            if let Err(error) = accept_connection(stream, rx, tjanx_service).await {
                eprintln!("{}", error);
            }
        });
    }
    Ok(())
}

async fn accept_connection(
    stream: TcpStream,
    mut rx: Receiver<String>,
    tjanx_service: TjanXService,
) -> Result<(), WebSocketConnectionError> {
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;
    info!("New WebSocket connection");
    ws_stream
        .send(Message::Text(tjanx_service.get_ranking().await?))
        .await?;
    while let Ok(value) = rx.recv().await {
        ws_stream.send(Message::Text(value.to_string())).await?;
    }
    Ok(())
}
