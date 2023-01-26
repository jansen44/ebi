use std::borrow::Borrow;
use std::ffi::{c_char, CString};
use std::path::PathBuf;

use ebi_source::prelude::{async_ffi::FfiFuture, serde_json};
use ebi_source::Source;

#[cfg(target_os = "macos")]
fn handle_source_file_extension(identifier: &str) -> PathBuf {
    let file = format!("lib{}.dylib", identifier);
    PathBuf::from(file)
}

#[cfg(target_os = "windows")]
fn handle_source_file_extension(identifier: &str) -> PathBuf {
    let file = format!("{}.dll", identifier);
    PathBuf::from(file)
}

#[cfg(target_os = "linux")]
fn handle_source_file_extension(identifier: &str) -> PathBuf {
    let file = format!("lib{}.so", identifier);
    PathBuf::from(file)
}

pub struct SourceManager {
    dir_path: PathBuf,
    pub loaded_sources: Vec<Source>,
}

impl SourceManager {
    pub fn new(dir_path: &str) -> Self {
        let dir_path = PathBuf::from(dir_path);

        Self {
            dir_path,
            loaded_sources: Vec::new(),
        }
    }

    // TODO: error handling
    pub async fn load_source(&mut self, identifier: &str) -> Result<(), String> {
        let file_name = handle_source_file_extension(identifier);

        let mut path = self.dir_path.clone();
        path.push(file_name);

        unsafe {
            use libloading::Library;

            let source_lib = Library::new(path).unwrap();
            let source_fn = source_lib
                .get::<extern "C" fn() -> FfiFuture<*mut c_char>>(b"abi_source")
                .unwrap();

            let source = source_fn().await;
            let source = CString::from_raw(source);
            let source = source.to_string_lossy();

            let source = serde_json::from_str(source.borrow()).unwrap();
            self.loaded_sources.push(source);

            let manga_fn = source_lib
                .get::<extern "C" fn() -> FfiFuture<*mut c_char>>(b"abi_manga_list")
                .unwrap();

            let manga = manga_fn().await;
            let manga = CString::from_raw(manga);
            let manga = manga.to_string_lossy();

            let manga_lst: Vec<ebi_source::Manga> = serde_json::from_str(manga.borrow()).unwrap();
            for manga in manga_lst.iter() {
                println!("{} -- {} -- {}", manga.identifier, manga.title, manga.cover);
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use ebi_source::locale::Locale;
    use pollster::FutureExt as _;

    #[test]
    fn load_opex_and_yabu_sources() {
        let mut source_manager = super::SourceManager::new("../target/debug");

        assert_eq!(source_manager.load_source("opex").block_on(), Ok(()));
        assert_eq!(source_manager.load_source("yabu").block_on(), Ok(()));

        assert_eq!(source_manager.loaded_sources.len(), 2);

        let opex_source = source_manager.loaded_sources.get(0).unwrap();
        let yabu_source = source_manager.loaded_sources.get(1).unwrap();

        assert_eq!(opex_source.identifier, "opex");
        assert_eq!(opex_source.title, "One Piece Ex");
        assert_eq!(opex_source.description, "One Piece Ex | De fã para fã");
        assert_eq!(opex_source.locale, Locale::PtBr);

        assert_eq!(yabu_source.identifier, "yabu");
        assert_eq!(yabu_source.title, "Manga Yabu");
        assert_eq!(yabu_source.description, "Manga Yabu! - Ler Mangás Online");
        assert_eq!(yabu_source.locale, Locale::PtBr);
    }
}
