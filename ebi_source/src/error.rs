use std::ffi::NulError;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SourceError {
    #[error("UNKNOWN_ERROR::{0}")]
    Unknown(String),
    #[error("COULD_NOT_FETCH_DATA")]
    Fetch,
    #[error("COULD_NOT_SERIALIZE_DATA")]
    Serialize,
    #[error("INVALID_IDENTIFIER_PROVIDED")]
    InvalidIdentifier,

    #[error("ABI_NULL_CONVERSION_ERROR")]
    ABINullConversion,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceErrorSerialized {
    pub error: SourceError,
}

impl std::convert::From<NulError> for SourceError {
    fn from(_: NulError) -> Self {
        Self::ABINullConversion
    }
}
