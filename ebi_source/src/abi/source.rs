use std::str::FromStr;

use crate::{error::SourceError, locale::Locale, Source};

use super::primitives::FFIString;

#[repr(C)]
pub struct ABISource {
    pub identifier: FFIString,
    pub title: FFIString,
    pub description: FFIString,
    pub locale: FFIString,
}

impl From<&Source> for ABISource {
    fn from(source: &Source) -> Self {
        let description = FFIString::from(source.description.clone());
        let identifier = FFIString::from(source.identifier.clone());
        let title = FFIString::from(source.title.clone());
        let locale = FFIString::from(source.locale.to_string().clone());

        Self {
            description,
            identifier,
            title,
            locale,
        }
    }
}

impl TryInto<Source> for ABISource {
    type Error = SourceError;

    fn try_into(self) -> Result<Source, Self::Error> {
        let description = self.description.try_into()?;
        let identifier = self.identifier.try_into()?;
        let title = self.title.try_into()?;

        let locale: String = self.locale.try_into()?;
        let locale = Locale::from_str(&locale).unwrap(); // safe to unwrap

        Ok(Source {
            description,
            identifier,
            title,
            locale,
        })
    }
}

pub mod source_info {
    use std::str::FromStr;

    use crate::{
        abi::primitives::{FFIArray, FFIString},
        error::SourceError,
        Source,
    };

    use super::ABISource;

    pub type SourceInfoFn = extern "C" fn() -> ABISourceInfoOutput;

    #[repr(C)]
    pub struct ABISourceInfoOutput {
        pub source: FFIArray,
        pub err: FFIString,
    }

    impl From<Result<Source, SourceError>> for ABISourceInfoOutput {
        fn from(result: Result<Source, SourceError>) -> Self {
            match result {
                Ok(source) => {
                    let source = FFIArray::from(vec![ABISource::from(&source)]);
                    Self {
                        source,
                        err: FFIString::null(),
                    }
                }
                Err(err) => Self {
                    source: FFIArray::null(),
                    err: FFIString::from(err.to_string()),
                },
            }
        }
    }

    impl Into<Result<Source, SourceError>> for ABISourceInfoOutput {
        fn into(self) -> Result<Source, SourceError> {
            if !self.err.is_null() {
                let err: String = self.err.try_into()?;
                return Err(SourceError::from_str(&err).unwrap()); // safe to unwrap
            }

            let mut abi_sources: Vec<ABISource> = self.source.try_into()?;

            match abi_sources.len() {
                0 => Err(SourceError::InvalidSource),
                _ => {
                    let source = abi_sources.remove(0).try_into()?;
                    Ok(source)
                }
            }
        }
    }
}
