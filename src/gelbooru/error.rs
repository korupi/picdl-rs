use crate::http::reqwest::ReqwestError;

impl From<ReqwestError> for GelbooruError {
    fn from(value: ReqwestError) -> Self {
        Self::HttpError(value)
    }
}

impl From<serde_json::Error> for GelbooruError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

#[derive(Debug)]
pub enum GelbooruError {
    HttpError(ReqwestError),
    SerdeError(serde_json::Error)
}
