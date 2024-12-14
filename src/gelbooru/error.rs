use crate::http::reqwest::ReqwestError;

/// Implements conversion of ReqwestError to GelbooruError
impl From<ReqwestError> for GelbooruError {
    fn from(value: ReqwestError) -> Self {
        Self::HttpError(value)
    }
}

/// Implements conversion of serde_json::Error to GelbooruError
impl From<serde_json::Error> for GelbooruError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

/// Enum of possible errors
#[derive(Debug)]
pub enum GelbooruError {
    HttpError(ReqwestError),
    SerdeError(serde_json::Error)
}
