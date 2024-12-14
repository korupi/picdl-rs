use std::{collections::HashMap, future::Future};

/// Universal alias of `HashMap<String, String>`
pub type Headers = HashMap<String, String>;
/// Universal alias of `HashMap<String, String>`
pub type Query = HashMap<String, String>;

/// A trait that can be implemented for any HTTP
/// client library in Rust so we can use any library
/// easily.
pub trait HttpClient: Send + Default {
    /// HttpClient error type
    type Error;

    /// Function for sending GET requests
    fn get(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;

    /// Function for sending POST requests
    fn post(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;
}
