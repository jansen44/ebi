use std::borrow::Borrow;
use std::ffi::CString;
use std::path::PathBuf;

use ebi_source::abi::{ABIChapterListInput, JSONInputedResourceFn, JSONResourceFn};
use ebi_source::prelude::{serde_json, SourceErrorSerialized};
use ebi_source::{Chapter, Manga, Source as EbiSource, SourceLoader};

use libloading::{Library, Symbol};

fn ptr_to_string(ptr: *mut i8) -> String {
    let stri = unsafe { CString::from_raw(ptr) };
    stri.to_string_lossy().to_string()
}

fn json_fn_to_string(f: Symbol<JSONResourceFn>) -> String {
    let f = f();
    ptr_to_string(f)
}

fn json_fn_to_string_inputed<T>(f: Symbol<JSONInputedResourceFn<T>>, input: T) -> String {
    let f = f(input);
    ptr_to_string(f)
}

pub struct Source {
    lib: Library,
    source: EbiSource,
}

// TODO: Better error handling
impl std::convert::TryFrom<PathBuf> for Source {
    type Error = String;

    fn try_from(source_path: PathBuf) -> Result<Self, Self::Error> {
        let lib = unsafe { Library::new(source_path) };
        let lib = match lib {
            Ok(lib) => lib,
            Err(e) => return Err(String::from(e.to_string())),
        };

        let source_fn = unsafe { lib.get::<JSONResourceFn>(b"abi_source") };
        let source = match source_fn {
            Ok(source_fn) => {
                let source = json_fn_to_string(source_fn);
                serde_json::from_str(source.borrow())
            }
            Err(e) => return Err(String::from(e.to_string())),
        };

        match source {
            Ok(source) => Ok(Source { lib, source }),
            Err(e) => Err(String::from(e.to_string())),
        }
    }
}

impl Source {
    fn get_abi_func_response(&self, name: &str) -> Result<String, String> {
        let f = unsafe { self.lib.get::<JSONResourceFn>(name.as_bytes()) };
        match f {
            Ok(f) => Ok(json_fn_to_string(f)),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_abi_inputed_func_response<T>(&self, name: &str, input: T) -> Result<String, String> {
        let f = unsafe { self.lib.get::<JSONInputedResourceFn<T>>(name.as_bytes()) };
        match f {
            Ok(f) => Ok(json_fn_to_string_inputed(f, input)),
            Err(e) => Err(e.to_string()),
        }
    }
}

// TODO: Better error handling
impl SourceLoader for Source {
    type Error = String;

    fn source(&self) -> Result<EbiSource, Self::Error> {
        Ok(self.source.clone())
    }

    fn manga_list(&self) -> Result<Vec<Manga>, Self::Error> {
        let manga_list = self
            .get_abi_func_response("abi_manga_list")
            .map_err(|err| err.to_string())?;

        let manga = serde_json::from_str(&manga_list);
        match manga {
            Ok(manga) => Ok(manga),
            Err(_) => {
                let err: SourceErrorSerialized =
                    serde_json::from_str(&manga_list).map_err(|err| err.to_string())?;
                Err(format!("{}", err.error))
            }
        }
    }

    fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>, Self::Error> {
        let manga = ABIChapterListInput::try_from(manga).unwrap();

        let chapter_list = self
            .get_abi_inputed_func_response("abi_chapter_list", manga)
            .map_err(|err| err.to_string())?;

        let chapters = serde_json::from_str(&chapter_list);
        match chapters {
            Ok(chapters) => Ok(chapters),
            Err(_) => {
                let err: SourceErrorSerialized =
                    serde_json::from_str(&chapter_list).map_err(|err| err.to_string())?;
                Err(format!("{}", err.error))
            }
        }
    }
}
