use log::debug;

use super::client::{Headers, HttpClient, Query};

/// A test HTTP client. Will just print debug information
#[derive(Default, Debug, Clone, Copy)]
pub struct TestClient {}

/// Implements a test HTTP client
/// Should just log request data at debug level
impl HttpClient for TestClient {
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
    type Error = String;

    /// GET function that implements a GET request in a specific HTTP client,
    /// such as reqwest
    #[inline]
    async fn get(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> Result<String, Self::Error> {
        pretty_env_logger::init();
        debug!("GET request to endpoint {url}");
        debug!("Headers: {:#?}", headers);
        debug!("Payload: {:#?}", payload);
        Err(String::from("Using a test HTTP client"))
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
        pretty_env_logger::init();
        debug!("POST request to endpoint {url}");
        debug!("Headers: {:#?}", headers);
        debug!("Payload: {:#?}", payload);
        Err(String::from("Using a test HTTP client"))
    }
}

