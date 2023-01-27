mod source;

use ebi_source::SourceLoader;
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
    sources: HashMap<String, Source>,
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
        if let Some(_) = self.get(identifier) {
            return Err(String::from("Duplicated Source"));
        }

        let file_name = handle_source_file_extension(identifier);

        let mut path = self.dir_path.clone();
        path.push(file_name);

        let source = Source::try_from(path)?;
        let identifier = source.source()?.identifier.clone();
        self.sources.insert(identifier, source);

        Ok(())
    }

    pub fn get(&self, identifier: &str) -> Option<&Source> {
        self.sources.get(identifier)
    }

    pub fn available_sources(&self) -> Vec<String> {
        self.sources.iter().map(|(key, _)| key.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ebi_source::locale::Locale;
    use ebi_source::SourceLoader;

    use pollster::FutureExt;

    #[test]
    fn load_opex_and_yabu_sources() {
        let mut source_manager = super::SourceManager::new("../target/debug");

        let opex_identifier = "opex";
        let yabu_identifier = "yabu";

        assert_eq!(source_manager.load_source(opex_identifier), Ok(()));
        assert_eq!(source_manager.load_source(yabu_identifier), Ok(()));

        assert_eq!(source_manager.sources.len(), 2);

        let opex_source = source_manager.get(opex_identifier).unwrap();
        let yabu_source = source_manager.get(yabu_identifier).unwrap();

        let opex_source = opex_source.source().unwrap();
        let yabu_source = yabu_source.source().unwrap();

        assert_eq!(opex_source.identifier, "opex");
        assert_eq!(opex_source.title, "One Piece Ex");
        assert_eq!(opex_source.description, "One Piece Ex | De fã para fã");
        assert_eq!(opex_source.locale, Locale::PtBr);

        assert_eq!(yabu_source.identifier, "yabu");
        assert_eq!(yabu_source.title, "Manga Yabu");
        assert_eq!(yabu_source.description, "Manga Yabu! - Ler Mangás Online");
        assert_eq!(yabu_source.locale, Locale::PtBr);

        let mut sources = source_manager.available_sources();
        sources.sort();

        assert_eq!(
            sources,
            vec![String::from(opex_identifier), String::from(yabu_identifier)]
        );
    }

    #[test]
    fn load_opex_manga_list() {
        let mut source_manager = super::SourceManager::new("../target/debug");

        let opex_identifier = "opex";

        let _ = source_manager.load_source(opex_identifier);

        let opex_source = source_manager.get(opex_identifier).unwrap();
        let manga_list = opex_source.manga_list().block_on();

        assert!(manga_list.is_ok());

        let manga_list = manga_list.unwrap();
        assert_eq!(manga_list.len(), 3);
    }
}
