use thiserror::Error;

use ebi_source::prelude::{SourceError, SourceErrorSerialized};

#[derive(Error, Debug)]
pub enum EbiError {
    #[error("UNKNOWN_ERROR::{0}")]
    Unknown(String),

    #[error("DUPLICATED_SOURCE")]
    DuplicatedSource,
    #[error("INVALID_SOURCE")]
    InvalidSource,

    #[error("COULD_NOT_LOAD_LIB")]
    LoadLib,
    #[error("COULD_NOT_LOAD_FUNCTION")]
    LoadFunction,
    #[error("SOURCE_RETURNED_ERRORED_RESPONSE")]
    SourceResponseError(SourceErrorSerialized),

    #[error("COULD_NOT_SERIALIZE_LIB_RESPONSE")]
    SerializeResponse,

    #[error("COULD_NOT_GENERATE_ABI_REPRESENTATION")]
    AbiSerialization,

    #[error("NO_CONTENT_TYPE_ON_RESPONSE_HEADERS")]
    NoContentType,
    #[error("UNSUPPORTED_FILE_FORMAT::{0}")]
    UnsupportedFile(String),
    #[error("COULD_NOT_READ_BUFFER")]
    CouldNotReadBuffer,
    #[error("COULD_NOT_SAVE_FILE::{0}")]
    CouldNotSaveFile(String),
    #[error("INVALID_REQUEST::{0}")]
    InvalidRequest(u16),
    #[error("COULD_NOT_COMPLETE_REQUEST")]
    CouldNotCompleteRequest,

    #[error("INVALID_DIR::{0}")]
    InvalidDir(String),
}

impl std::convert::From<SourceError> for EbiError {
    fn from(value: SourceError) -> Self {
        match value {
            SourceError::Unknown(msg) => Self::Unknown(msg),
            SourceError::Fetch => todo!(),
            SourceError::Serialize => todo!(),
            SourceError::InvalidIdentifier => todo!(),
            SourceError::ABINullConversion => Self::AbiSerialization,
        }
    }
}

impl std::convert::From<ureq::Error> for EbiError {
    fn from(value: ureq::Error) -> Self {
        match value {
            ureq::Error::Status(code, _) => Self::InvalidRequest(code),
            ureq::Error::Transport(_) => Self::CouldNotCompleteRequest,
        }
    }
}
