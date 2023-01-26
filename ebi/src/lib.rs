mod source;

use std::{collections::HashMap, path::PathBuf};

use source::Source;

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
    pub sources: HashMap<String, Source>,
}

impl SourceManager {
    pub fn new(dir_path: &str) -> Self {
        let dir_path = PathBuf::from(dir_path);

        Self {
            dir_path,
            sources: HashMap::new(),
        }
    }

    pub fn load_source(&mut self, identifier: &str) -> Result<(), String> {
        let file_name = handle_source_file_extension(identifier);

        let mut path = self.dir_path.clone();
        path.push(file_name);

        let source = Source::try_from(path).unwrap();
        let identifier = source.source.identifier.clone();
        self.sources.insert(identifier, source);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ebi_source::locale::Locale;

    #[test]
    fn load_opex_and_yabu_sources() {
        let mut source_manager = super::SourceManager::new("../target/debug");

        let opex_identifier = "opex";
        let yabu_identifier = "yabu";

        assert_eq!(source_manager.load_source(opex_identifier), Ok(()));
        assert_eq!(source_manager.load_source(yabu_identifier), Ok(()));

        assert_eq!(source_manager.sources.len(), 2);

        let opex_source = source_manager.sources.get(opex_identifier).unwrap();
        let yabu_source = source_manager.sources.get(yabu_identifier).unwrap();

        let opex_source = opex_source.source.clone();
        let yabu_source = yabu_source.source.clone();

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
