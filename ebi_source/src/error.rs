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
    #[error("INVALID_SOURCE_PROVIDED")]
    InvalidSource,

    #[error("ABI_NULL_CONVERSION_ERROR")]
    ABINullConversion,
    #[error("ABI_RESULT_ERROR::{0}")]
    ABIResult(String),
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

impl std::str::FromStr for SourceError {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COULD_NOT_FETCH_DATA" => Ok(Self::Fetch),
            "COULD_NOT_SERIALIZE_DATA" => Ok(Self::Serialize),
            "INVALID_IDENTIFIER_PROVIDED" => Ok(Self::InvalidIdentifier),
            "INVALID_SOURCE_PROVIDED" => Ok(Self::InvalidSource),
            _ => Ok(Self::Unknown(s.to_string())),
        }
    }
}
