use std::borrow::Borrow;
use std::ffi::CString;
use std::path::PathBuf;

use ebi_source::prelude::{serde_json, ABIManga, JSONResourceFn};
use ebi_source::{Chapter, Manga, Source as EbiSource, SourceLoader};

use libloading::{Library, Symbol};

fn json_fn_to_string(f: Symbol<JSONResourceFn>) -> String {
    let f = f();
    let f = unsafe { CString::from_raw(f) };
    f.to_string_lossy().to_string()
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
            Err(e) => {
                return Err(e.to_string());
            }
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
        match self.get_abi_func_response("abi_manga_list") {
            Ok(manga_list) => {
                serde_json::from_str(manga_list.as_str()).map_err(|err| err.to_string())
            }
            Err(e) => Err(e),
        }
    }

    fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>, Self::Error> {
        let f = unsafe {
            self.lib
                .get::<extern "C" fn(manga: ABIManga) -> *mut std::ffi::c_char>(b"abi_chapter_list")
                .unwrap()
        };
        let manga: ABIManga = manga.into();
        let f = f(manga);
        let f = unsafe { CString::from_raw(f) };
        let f = f.to_string_lossy().to_string();

        serde_json::from_str(f.as_str()).map_err(|err| err.to_string())
    }
}
