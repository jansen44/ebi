use thiserror::Error;

use ebi_source::prelude::{SourceError, SourceErrorSerialized};

#[derive(Error, Debug)]
pub enum EbiError {
    #[error("UNKNOWN_ERROR::{0}")]
    Unknown(String),

    #[error("DUPLICATED_SOURCE")]
    DuplicatedSource,

    #[error("COULD_NOT_LOAD_LIB")]
    LoadLib,
    #[error("COULD_NOT_LOAD_FUNCTION")]
    LoadFunction,
    #[error("COULD_NOT_LOAD_MANGA_LIST::{0:?}")]
    LoadMangaList(SourceErrorSerialized),
    #[error("COULD_NOT_LOAD_CHAPTER_LIST::{0:?}")]
    LoadChapterList(SourceErrorSerialized),

    #[error("COULD_NOT_SERIALIZE_LIB_RESPONSE")]
    SerializeResponse,

    #[error("COULD_NOT_GENERATE_ABI_REPRESENTATION")]
    AbiSerialization,
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
