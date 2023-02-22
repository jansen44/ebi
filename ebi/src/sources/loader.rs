use std::path::PathBuf;

use libloading::Library;

use ebi_source::abi::{ABIChapterListInput, JSONInputedResourceFn, JSONResourceFn};
use ebi_source::prelude::{
    serde_json, ABIChapterPageListInput, Deserialize, SourceErrorSerialized,
};

use crate::error::EbiError;

use super::{ptr_to_string, EbiChapter, EbiManga, EbiSource, SourceLoader};

macro_rules! abi_fn_response_to_string {
    ($f:expr) => {{
        let response = $f();
        log::debug!("No-args source fn response from {:?}", $f);
        ptr_to_string(response)
    }};
    ($f:expr, $input:expr) => {{
        let response = $f($input);
        log::debug!("Args source fn response from {:?}", $f);
        ptr_to_string(response)
    }};
}

struct AbiFn<'a, T> {
    name: &'a str,
    arg: Option<T>,
}

pub struct Source {
    lib: Library,
    source: EbiSource,
}

impl std::convert::TryFrom<PathBuf> for Source {
    type Error = EbiError;

    fn try_from(source_path: PathBuf) -> Result<Self, Self::Error> {
        let lib = unsafe { Library::new(source_path.clone()).map_err(|_| EbiError::LoadLib)? };
        log::debug!("Loaded Source from {}", source_path.display());

        let source_fn = unsafe {
            lib.get::<JSONResourceFn>(b"abi_source")
                .map_err(|_| EbiError::LoadFunction)?
        };

        let source = abi_fn_response_to_string!(source_fn);
        let source = deserialize(&source)?;
        Ok(Source { lib, source })
    }
}

impl Source {
    fn exec_abi_fn<T>(&self, abi_fn: AbiFn<'_, T>) -> Result<String, EbiError> {
        let f = abi_fn.name.as_bytes();
        unsafe {
            match abi_fn.arg {
                Some(arg) => {
                    let f = self.lib.get::<JSONInputedResourceFn<T>>(f);
                    match f {
                        Ok(f) => Ok(abi_fn_response_to_string!(f, arg)),
                        Err(_) => Err(EbiError::LoadFunction),
                    }
                }
                None => {
                    let f = self.lib.get::<JSONResourceFn>(f);
                    match f {
                        Ok(f) => Ok(abi_fn_response_to_string!(f)),
                        Err(_) => Err(EbiError::LoadFunction),
                    }
                }
            }
        }
    }
}

// TODO: Generate fns with macro as they're very similar to each other (???)
impl SourceLoader for Source {
    type Error = EbiError;

    fn source(&self) -> Result<EbiSource, Self::Error> {
        Ok(self.source.clone())
    }

    fn manga_list(&self) -> Result<Vec<EbiManga>, Self::Error> {
        let manga_list = self.exec_abi_fn::<()>(AbiFn {
            name: "abi_manga_list",
            arg: None,
        })?;
        deserialize(&manga_list)
    }

    fn chapter_list(&self, manga: &EbiManga) -> Result<Vec<EbiChapter>, Self::Error> {
        let manga = ABIChapterListInput::try_from(manga)?;
        let chapter_list = self.exec_abi_fn(AbiFn {
            name: "abi_chapter_list",
            arg: Some(manga),
        })?;
        deserialize(&chapter_list)
    }

    fn chapter_page_list(&self, chapter: &EbiChapter) -> Result<Vec<String>, Self::Error> {
        let chapter = ABIChapterPageListInput::try_from(chapter)?;
        let chapter_page_list = self.exec_abi_fn(AbiFn {
            name: "abi_chapter_page_list",
            arg: Some(chapter),
        })?;
        deserialize(&chapter_page_list)
    }
}

fn deserialize<'a, T: Deserialize<'a>>(response: &'a str) -> Result<T, EbiError> {
    use serde_json::from_str;
    use EbiError::{SerializeResponse, SourceResponseError};

    let serialized = from_str(&response);
    match serialized {
        Ok(serialized) => Ok(serialized),
        Err(_) => {
            let err: SourceErrorSerialized = from_str(&response).map_err(|_| SerializeResponse)?;
            Err(SourceResponseError(err))
        }
    }
}
