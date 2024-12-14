use reqwest::{Client, Method};

use super::client::{Headers, HttpClient, Query};

/// Reqwest implementation of HttpClient
#[derive(Debug, Clone)]
pub struct ReqwestClient {
    client: Client
}

/// Enum of possible HTTP errors
#[derive(Debug)]
pub enum ReqwestError {
    Client(reqwest::Error),
    StatusCode(reqwest::Response),
}

impl Default for ReqwestClient {
    fn default() -> Self {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();
        Self { client }
    }
}

impl From<reqwest::Error> for ReqwestError {
    fn from(value: reqwest::Error) -> Self {
        ReqwestError::Client(value)
    }
}

/// Only implements `request` function for internal
/// usage in ReqwestClient
impl ReqwestClient {

    /// Internal function to remove the sending request
    /// overhead from `get` and `post` functions 
    #[inline]
    async fn request(
        &self,
        payload: &Query,
        headers: &Headers,
        url: String,
        method: Method
    ) -> Result<String, ReqwestError> {
        let mut request = self.client.request(method, url);
        request = request
            .query(payload)
            .headers(headers.try_into().unwrap()) // Should not fail; converting from a
                                                  // HashMap<String, String> to a HeaderMap which
                                                  // is technically the same
            .header("Content-Type", "application/json");

        let response = request.send().await?;

        Ok(response.text().await?)
    }
}

/// Implements all functions of HttpClient for reqwest
impl HttpClient for ReqwestClient {
    /// Error type for a HTTP client
    /// Typically should be an enum of client errors
    ///
    /// # Example
    /// ```no_run
    /// #[derive(Debug)]
    /// pub enum ReqwestError {
    ///     Client(reqwest::Error),
    ///     StatusCode(reqwest::Response),
    /// }
    /// ```
    type Error = ReqwestError;

    /// GET function that implements a GET request in a specific HTTP client,
    /// such as reqwest
    #[inline]
    async fn get(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> Result<String, Self::Error> {
        self.request(payload, headers, url, Method::GET).await
    }

    /// POST function that implements a POST request in a specific HTTP client,
    /// such as reqwest
    #[inline]
    async fn post(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> Result<String, Self::Error> {
        self.request(payload, headers, url, Method::POST).await
    }
}

