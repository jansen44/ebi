use std::path::PathBuf;

pub use ebi_source::Source;

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
    let file = format!("{}.so", identifier);
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
    pub fn load_source(&mut self, identifier: &str) -> Result<(), String> {
        let file_name = handle_source_file_extension(identifier);

        let mut path = self.dir_path.clone();
        path.push(file_name);

        unsafe {
            use libloading::Library;

            let source_lib = Library::new(path).unwrap();
            let source_fn = source_lib
                .get::<extern "C" fn() -> Source>(b"source")
                .unwrap();

            let source = source_fn();

            self.loaded_sources.push(source);

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use ebi_source::locale::Locale;

    #[test]
    fn load_opex_and_yabu_sources() {
        let mut source_manager = super::SourceManager::new("../target/debug");
        assert_eq!(source_manager.load_source("opex"), Ok(()));
        assert_eq!(source_manager.load_source("yabu"), Ok(()));

        assert!(!source_manager.loaded_sources.is_empty());

        let opex_source = source_manager.loaded_sources.get(0).unwrap();
        let yabu_source = source_manager.loaded_sources.get(1).unwrap();

        let identifier = unsafe { CString::from_raw(opex_source.identifier) };
        let title = unsafe { CString::from_raw(opex_source.title) };
        let description = unsafe { CString::from_raw(opex_source.description) };

        let identifier = identifier.to_string_lossy();
        let title = title.to_string_lossy();
        let description = description.to_string_lossy();
        let locale = Locale::from(opex_source.locale);

        assert_eq!(identifier, "opex");
        assert_eq!(title, "One Piece Ex");
        assert_eq!(description, "One Piece Ex | De fã para fã");
        assert_eq!(locale, Locale::PtBr);

        let identifier = unsafe { CString::from_raw(yabu_source.identifier) };
        let title = unsafe { CString::from_raw(yabu_source.title) };
        let description = unsafe { CString::from_raw(yabu_source.description) };

        let identifier = identifier.to_string_lossy();
        let title = title.to_string_lossy();
        let description = description.to_string_lossy();
        let locale = Locale::from(yabu_source.locale);

        assert_eq!(identifier, "yabu");
        assert_eq!(title, "Manga Yabu");
        assert_eq!(description, "Manga Yabu! - Ler Mangás Online");
        assert_eq!(locale, Locale::PtBr);
    }
}
