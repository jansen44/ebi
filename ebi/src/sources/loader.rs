use std::ffi::CString;
use std::path::PathBuf;

use libloading::Library;

use ebi_source::abi::{ABIChapterListInput, JSONInputedResourceFn, JSONResourceFn};
use ebi_source::prelude::{
    serde_json, ABIChapterPageListInput, Deserialize, SourceErrorSerialized,
};

use crate::error::EbiError;

use super::{EbiChapter, EbiManga, EbiSource, SourceLoader};

fn ptr_to_string(ptr: *mut i8) -> String {
    let string = unsafe { CString::from_raw(ptr) };
    string.to_string_lossy().to_string()
}

macro_rules! abi_fn_to_string {
    ($f:expr) => {{
        let response = $f();
        ptr_to_string(response)
    }};
    ($f:expr, $input:expr) => {{
        let response = $f($input);
        ptr_to_string(response)
    }};
}

fn deserialize<'a, T: Deserialize<'a>>(response: &'a str) -> Result<T, EbiError> {
    use serde_json::from_str;
    use EbiError::{LoadChapterList, SerializeResponse};

    let serialized = from_str(&response);
    match serialized {
        Ok(serialized) => Ok(serialized),
        Err(_) => {
            let err: SourceErrorSerialized = from_str(&response).map_err(|_| SerializeResponse)?;
            Err(LoadChapterList(err))
        }
    }
}

struct AbiFn<'a, T> {
    name: &'a str,
    arg: Option<T>,
}

pub struct Source {
    lib: Library,
    source: EbiSource,
}

impl Source {
    fn exec_abi_fn<T>(&self, abi_fn: AbiFn<'_, T>) -> Result<String, EbiError> {
        let f = abi_fn.name.as_bytes();
        unsafe {
            match abi_fn.arg {
                // With Arg
                Some(arg) => {
                    let f = self.lib.get::<JSONInputedResourceFn<T>>(f);
                    match f {
                        Ok(f) => Ok(abi_fn_to_string!(f, arg)),
                        Err(_) => Err(EbiError::LoadFunction),
                    }
                }
                // Without Arg
                None => {
                    let f = self.lib.get::<JSONResourceFn>(f);
                    match f {
                        Ok(f) => Ok(abi_fn_to_string!(f)),
                        Err(_) => Err(EbiError::LoadFunction),
                    }
                }
            }
        }
    }
}

// TODO: Add logs
impl std::convert::TryFrom<PathBuf> for Source {
    type Error = EbiError;

    fn try_from(source_path: PathBuf) -> Result<Self, Self::Error> {
        let lib = unsafe { Library::new(source_path).map_err(|_| EbiError::LoadLib)? };

        let source_fn = unsafe {
            lib.get::<JSONResourceFn>(b"abi_source")
                .map_err(|_| EbiError::LoadFunction)?
        };

        let source = abi_fn_to_string!(source_fn);
        let source = serde_json::from_str(&source);

        match source {
            Ok(source) => Ok(Source { lib, source }),
            Err(_) => Err(EbiError::SerializeResponse),
        }
    }
}

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
