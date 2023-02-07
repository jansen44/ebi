use thiserror::Error;

use ebi_source::prelude::SourceErrorSerialized;

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
}
