use std::borrow::Borrow;
use std::ffi::CString;
use std::path::PathBuf;

use ebi_source::prelude::{serde_json, AsyncJSONResourceFn, JSONResourceFn};
use ebi_source::{Chapter, Manga, Source as EbiSource, SourceLoader};

use libloading::{Library, Symbol};

fn json_fn_to_string(f: Symbol<JSONResourceFn>) -> String {
    let f = f();
    let f = unsafe { CString::from_raw(f) };
    f.to_string_lossy().to_string()
}

async fn async_json_fn_to_string(f: Symbol<'_, AsyncJSONResourceFn>) -> String {
    let f = f().await;
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

// TODO: Better error handling
#[async_trait::async_trait]
impl SourceLoader for Source {
    type Error = String;

    fn source(&self) -> Result<EbiSource, Self::Error> {
        Ok(self.source.clone())
    }

    async fn manga_list(&self) -> Result<Vec<Manga>, Self::Error> {
        let manga_list_fn = unsafe { self.lib.get::<AsyncJSONResourceFn>(b"abi_manga_list") };
        let manga_list = match manga_list_fn {
            Ok(manga_list) => {
                let manga_list = async_json_fn_to_string(manga_list).await;
                serde_json::from_str(manga_list.borrow())
            }
            Err(e) => {
                return Err(e.to_string());
            }
        };
        manga_list.map_err(|err| err.to_string())
    }

    async fn latest_manga(&self) -> Result<Vec<Manga>, Self::Error> {
        todo!()
    }

    async fn popular_manga(&self) -> Result<Vec<Manga>, Self::Error> {
        todo!()
    }

    async fn hot_manga(&self) -> Result<Vec<Manga>, Self::Error> {
        todo!()
    }

    async fn search_manga(&self, _manga_title: &str) -> Result<Vec<Manga>, Self::Error> {
        todo!()
    }

    async fn get_manga(&self, _manga_identifier: &str) -> Result<Manga, Self::Error> {
        todo!()
    }

    async fn chapter_list(&self, _manga: Manga) -> Result<Vec<Chapter>, Self::Error> {
        todo!()
    }

    async fn chapter(
        &self,
        _manga: Manga,
        _chapter: usize,
    ) -> Result<Option<Chapter>, Self::Error> {
        todo!()
    }

    async fn page_url_list(&self, _chapter: Chapter) -> Result<Vec<String>, Self::Error> {
        todo!()
    }
}
