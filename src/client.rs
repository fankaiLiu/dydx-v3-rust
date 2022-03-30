use super::{Api, Error, Result};
use http::StatusCode;
use reqwest::RequestBuilder;
use serde_json::{Value};
use std::time::Duration;

pub struct Client {
    client: reqwest::Client,
    api: Api,
}

#[derive(Debug)]
pub struct Response {
    pub response: reqwest::Response,
    pub request: reqwest::Request,
}

impl Response {
    pub async fn json(self) -> Result<Value> {
        Ok(self.response.json().await?)
    }
}

impl Client {
    pub fn new(env: &str) -> Client {
        Client {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            api: if env == "production" {
                Api::Production
            } else {
                Api::Staging
            },
        }
    }

    pub async fn get_markets(&self, parameters: &Value) -> Result<String> {
        self.get("markets", parameters).await
    }

    pub async fn get_orderbook(&self, market: &str) -> Result<String> {
        let url="orderbook".to_string()+"/"+market;
        self.get_no_query(&url).await
    }
    pub async fn get_trades(&self, market: &str) -> Result<String> {
        let url="trades".to_string()+"/"+market;
        self.get_no_query(&url).await
    }
    pub async fn get(&self, endpoint: &str, parameters: &Value) -> Result<String> {
        let request = self
            .client
            .get(format!("{}/{}", self.api.url(), endpoint))
            .query(parameters);

        Ok(self.request(request).await?)
    }
    pub async fn get_no_query(&self, endpoint: &str) -> Result<String> {
        let request = self
            .client
            .get(format!("{}/{}", self.api.url(), endpoint));
        Ok(self.request(request).await?)
    }
    async fn request(&self, request: RequestBuilder) -> Result<String> {
        let request = request.build()?;

        let response = self
            .client
            .execute(request.try_clone().expect(
                "Error can remain unhandled because we're not using streams, which are the try_clone fail condition",
            ))
            .await;

        match &response {
            Ok(response) => match response.status() {
                StatusCode::NOT_FOUND => return Err(Error::NotFoundError),
                StatusCode::UNAUTHORIZED => return Err(Error::AuthenticationError),
                StatusCode::BAD_REQUEST => return Err(Error::InvalidRequestError),
                _ => {}
            },
            Err(err) => {
                if err.is_connect() || err.is_timeout() {
                    return Err(Error::ApiConnectionError);
                }
                else
                {
                    return Err(Error::ApiError);
                }
            }
        };
        let response = response.unwrap().text().await;
        match response {
            Ok(response) => Ok(response),
            Err(_) => Err(Error::ApiError),
        }
    }
}
