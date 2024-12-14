use std::{collections::HashMap, future::Future};


pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;

/// A trait that can be implemented for any HTTP
/// client library in Rust so we can use any library
/// easily.
pub trait HttpClient: Send + Default {
    type Error;

   fn get(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;

    fn post(
        &self,
        url: String,
        headers: &Headers,
        payload: &Query,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;
}
