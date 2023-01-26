use std::borrow::Borrow;
use std::ffi::CString;
use std::path::PathBuf;

use ebi_source::prelude::{serde_json, JSONResourceFn};
use ebi_source::Source as EbiSource;

use libloading::{Library, Symbol};

fn json_fn_to_string(f: Symbol<JSONResourceFn>) -> String {
    let f = f();
    let f = unsafe { CString::from_raw(f) };
    f.to_string_lossy().to_string()
}

pub struct Source {
    _lib: Library,
    pub source: EbiSource,
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
            Ok(source) => Ok(Source { _lib: lib, source }),
            Err(e) => Err(String::from(e.to_string())),
        }
    }
}

// TODO: Better error handling
// #[async_trait::async_trait]
// impl SourceLoader for Source {
//     type Error = String;

//     async fn manga_list(&self) -> Result<Vec<Manga>, Self::Error> {
//         todo!()
//     }

//     async fn latest_manga(&self) -> Result<Vec<Manga>, Self::Error> {
//         todo!()
//     }

//     async fn popular_manga(&self) -> Result<Vec<Manga>, Self::Error> {
//         todo!()
//     }

//     async fn hot_manga(&self) -> Result<Vec<Manga>, Self::Error> {
//         todo!()
//     }

//     async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>, Self::Error> {
//         todo!()
//     }

//     async fn get_manga(&self, manga_identifier: &str) -> Result<Manga, Self::Error> {
//         todo!()
//     }

//     async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>, Self::Error> {
//         todo!()
//     }

//     async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>, Self::Error> {
//         todo!()
//     }

//     async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>, Self::Error> {
//         todo!()
//     }
// }
