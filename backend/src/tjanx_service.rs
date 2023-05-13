use crate::errors::{get_ranking_error::GetRankingError, send_error::SendError};
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::time::Duration;
use tokio::sync::broadcast::Sender;
#[derive(Clone)]
pub struct TjanXService {
    api_key: String,
    client: Option<Client<HttpsConnector<HttpConnector>>>,
}

impl TjanXService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Some(Client::builder().build::<_, hyper::Body>(HttpsConnector::new())),
        }
    }

    pub async fn get_ranking(&self) -> Result<String, GetRankingError> {
        if let Some(client) = &self.client {
            let request: Request<_> = Request::builder()
                .method(Method::GET)
                .uri("https://portal.tjan.tv/api/votings/tjanx")
                .header("Authorization", self.api_key.clone())
                .body(Body::empty())?;
            let response = client.request(request).await?;
            let response_string =
                String::from_utf8(to_bytes(response.into_body()).await?.to_vec())?;
            Ok(response_string)
        } else {
            Err(GetRankingError::ClientNotInitialized)
        }
    }

    pub async fn send_rankings(
        &self,
        tx: Sender<String>,
        fetch_interval: Duration,
    ) -> Result<(), SendError> {
        loop {
            tx.send(self.get_ranking().await?)?;
            tokio::time::sleep(fetch_interval).await;
        }
    }
}
