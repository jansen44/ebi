use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SourceError {
    #[error("UNKNOWN_ERROR::{0}")]
    Unknown(String),
    #[error("COULD_NOT_FETCH_DATA")]
    Fetch,
}

#[derive(Deserialize, Serialize)]
pub struct SourceErrorSerialized {
    pub error: SourceError,
}
