use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("UNKNOWN_ERROR")]
    Unknown,
}

impl From<u8> for SourceError {
    fn from(value: u8) -> Self {
        match value {
            _ => Self::Unknown,
        }
    }
}

impl Into<u8> for SourceError {
    fn into(self) -> u8 {
        match self {
            Self::Unknown => 0,
        }
    }
}
