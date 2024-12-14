pub mod error;
pub mod types;

use log::debug;
use types::Response;
use crate::http::{client::{Headers, Query}, reqwest::ReqwestError};

use super::http::{client::HttpClient, reqwest::ReqwestClient};

/// Boosty client struct
///
/// # Example
///
/// Using an async http client (reqwest)
/// ```no_test
/// let client = picdl_rs::boosty::Boosty::<picdl_rs::http::reqwest::ReqwestClient>::new();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Boosty<Http: HttpClient> {
    http: Http,
}

#[derive(Debug)]
pub enum BoostyError {
    HttpError(ReqwestError),
    SerdeError(serde_json::Error)
}

impl From<ReqwestError> for BoostyError {
    fn from(value: ReqwestError) -> Self {
        Self::HttpError(value)
    }
}

impl From<serde_json::Error> for BoostyError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

/// Boosty API client
impl<Http: HttpClient> Boosty<Http>
    where BoostyError: From<<Http as HttpClient>::Error> {
    /// Returns a Gelbooru client with ReqwestClient as HTTP client
    pub fn new() -> Boosty<ReqwestClient> {
        Boosty{ http: <ReqwestClient as Default>::default() }
    }

    pub async fn fetch(&self, blog: &str, limit: i64) -> Result<Response, BoostyError> {
        let headers = Headers::new();
        let payload = Query::new();
        debug!("Requesting {}", format!("https://api.boosty.to/v1/blog/{blog}/post/?limit={limit}"));
        let response = self.http.get(
            format!("https://api.boosty.to/v1/blog/{blog}/post/?limit={limit}"),
            &headers,
            &payload).await?;

        let json: Response = serde_json::from_str(&response)?;
        Ok(json) 
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use crate::boosty::Boosty;
    use crate::http::reqwest::ReqwestClient;

    #[tokio::test]
    async fn test_gelbooru_fetch() {
        let _ = pretty_env_logger::try_init();
        let client = Boosty::<ReqwestClient>::new();
        let result = client.fetch("boosty", 1);
        debug!("Post title: {:?}", result.await.unwrap().data[0].title);
    }
}
