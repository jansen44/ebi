use std::{collections::HashMap, path::PathBuf};

use ebi_source::REGISTER_FUNCTION_NAME;
pub use ebi_source::{Chapter, Manga, Source};

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
    loaded_sources: HashMap<String, Box<dyn Source>>,
}

impl SourceManager {
    pub fn new(dir_path: &str) -> Self {
        let dir_path = PathBuf::from(dir_path);

        Self {
            dir_path,
            loaded_sources: HashMap::new(),
        }
    }

    // TODO: error handling
    pub fn load_source(&mut self, identifier: &str) -> Result<(), String> {
        use libloading::{Library, Symbol};
        use std::borrow::Cow;

        let file_name = handle_source_file_extension(identifier);

        let mut path = self.dir_path.clone();
        path.push(file_name);

        unsafe {
            let source = Library::new(path).unwrap();
            let register_func: Symbol<unsafe extern "C" fn() -> Box<dyn Source>> =
                source.get(REGISTER_FUNCTION_NAME).unwrap();

            let source = register_func();
            let identifier = match source.identifier() {
                Cow::Owned(identifier) => identifier.clone(),
                Cow::Borrowed(identifier) => identifier.to_string(),
            };

            self.loaded_sources.insert(identifier, source);

            Ok(())
        }
    }

    pub fn get_source(&self, identifier: &str) -> Option<&Box<dyn Source>> {
        let source = self.loaded_sources.get(identifier);
        match source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_opex_and_yabu_sources() {
        let mut source_manager = super::SourceManager::new("../target/debug");
        assert_eq!(source_manager.load_source("opex"), Ok(()));
        assert_eq!(source_manager.load_source("yabu"), Ok(()));

        let opex_source = source_manager.get_source("opex");
        let yabu_source = source_manager.get_source("yabu");
        assert!(opex_source.is_some());
        assert!(yabu_source.is_some());

        let opex_identifier = opex_source.unwrap().identifier();
        let opex_identifier = opex_identifier.as_ref();
        assert_eq!(opex_identifier, "opex");

        let yabu_identifier = yabu_source.unwrap().identifier();
        let yabu_identifier = yabu_identifier.as_ref();
        assert_eq!(yabu_identifier, "yabu");
    }
}
