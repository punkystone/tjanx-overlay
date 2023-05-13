pub mod env;
mod errors;
mod tjanx_service;
pub mod websocket_service;

use std::time::Duration;

use errors::run_error::RunError;

use tjanx_service::TjanXService;

use tokio::sync::broadcast::channel;
use tracing::{error, warn};
use websocket_service::start_websocket;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(e) = run().await {
        error!("Error: {}", e);
    }
}

async fn run() -> Result<(), RunError> {
    let env = env::Env::check_variables()?;
    let tjanx_service = TjanXService::new(env.api_key);
    let tjanx_service_sender = tjanx_service.clone();
    let (tx, rx) = channel::<String>(1);

    tokio::spawn(async move {
        if let Err(error) = tjanx_service_sender
            .send_rankings(tx, Duration::from_secs(env.fetch_interval))
            .await
        {
            warn!("Error: {}", error);
        }
    });
    start_websocket(rx, tjanx_service, env.port).await?;
    Ok(())
}
