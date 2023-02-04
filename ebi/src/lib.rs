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
mod valid_tests {
    use ebi_source::SourceLoader;
    use valid_source_macro_mock::{chapter_list, manga_list, source};

    const SOURCE_DIR: &str = "../target/debug";

    #[test]
    fn load_valid_source() {
        let mock_source = source();

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&mock_source.identifier).unwrap();

        let dy_source = source_manager
            .get(&mock_source.identifier)
            .unwrap()
            .source()
            .unwrap();
        assert_eq!(dy_source.identifier, mock_source.identifier);
        assert_eq!(dy_source.title, mock_source.title);
        assert_eq!(dy_source.description, mock_source.description);
        assert_eq!(dy_source.locale, mock_source.locale);

        assert_eq!(
            source_manager.available_sources(),
            vec![dy_source.identifier]
        );
    }

    #[test]
    fn load_valid_manga_list() {
        let mock_source = source();
        let mock_manga_list = manga_list().unwrap();

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&mock_source.identifier).unwrap();

        let dy_manga_list = source_manager
            .get(&mock_source.identifier)
            .unwrap()
            .manga_list()
            .unwrap();
        assert_eq!(mock_manga_list.len(), dy_manga_list.len());

        for (mock, dy) in mock_manga_list.iter().zip(dy_manga_list.iter()) {
            assert_eq!(mock.identifier, dy.identifier);
            assert_eq!(mock.title, dy.title);
            assert_eq!(mock.url, dy.url);
            assert_eq!(mock.cover, dy.cover);
            assert_eq!(mock.genres, dy.genres);
            assert_eq!(mock.description, dy.description);
            assert_eq!(mock.source_identifier, dy.source_identifier);
        }
    }

    #[test]
    fn load_valid_chapter_list() {
        let mock_source = source();
        let mock_manga_list = manga_list().unwrap();

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&mock_source.identifier).unwrap();

        let dy_source = source_manager.get(&mock_source.identifier).unwrap();

        for manga in mock_manga_list.iter() {
            let mock_chapter_list = chapter_list(manga.clone()).unwrap();
            let dy_chapter_list = dy_source.chapter_list(manga.clone()).unwrap();
            assert_eq!(mock_chapter_list.len(), dy_chapter_list.len());

            for (mock, dy) in mock_chapter_list.iter().zip(dy_chapter_list.iter()) {
                assert_eq!(mock.chapter, dy.chapter);
                assert_eq!(mock.title, dy.title);
                assert_eq!(mock.url, dy.url);
                assert_eq!(mock.manga_identifier, dy.manga_identifier);
                assert_eq!(mock.source_identifier, dy.source_identifier);
            }
        }
    }
}

#[cfg(test)]
mod broken_tests {
    use ebi_source::{error::SourceError, SourceLoader};
    use invalid_source_macro_mock::{chapter_list, invalid_manga, source, valid_manga};

    const SOURCE_DIR: &str = "../target/debug";

    #[test]
    fn cant_load_invalid_source() {
        let invalid_source = "invalid_source_identifier";

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        assert!(source_manager.load_source(invalid_source).is_err());
    }

    #[test]
    fn cant_same_valid_source_multiple_times() {
        let source = source();

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        assert!(source_manager.load_source(&source.identifier).is_ok());
        assert!(source_manager.load_source(&source.identifier).is_err());
        assert!(source_manager.load_source(&source.identifier).is_err());
    }

    #[test]
    fn cant_load_manga_list() {
        let source = source();

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&source.identifier).unwrap();
        let source = source_manager.get(&source.identifier).unwrap();

        let err_manga_list = source.manga_list();

        assert!(err_manga_list.is_err());
        assert_eq!(err_manga_list.unwrap_err(), SourceError::Fetch.to_string());
    }

    #[test]
    fn cant_load_chapter_list() {
        let source = source();
        let manga = valid_manga();
        let invalid_manga = invalid_manga();

        let mock_chapter_list = chapter_list(manga.clone()).unwrap();
        let mock_invalid_chapter_list = chapter_list(invalid_manga.clone());

        let mut source_manager = super::SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&source.identifier).unwrap();
        let source = source_manager.get(&source.identifier).unwrap();

        let valid_chapter_list = source.chapter_list(manga.clone()).unwrap();

        for (mock, dy) in mock_chapter_list.iter().zip(valid_chapter_list.iter()) {
            assert_eq!(mock.chapter, dy.chapter);
            assert_eq!(mock.title, dy.title);
            assert_eq!(mock.url, dy.url);
            assert_eq!(mock.manga_identifier, dy.manga_identifier);
            assert_eq!(mock.source_identifier, dy.source_identifier);
        }

        let err_chapter_list = source.chapter_list(invalid_manga);
        assert!(err_chapter_list.is_err());

        assert_eq!(
            err_chapter_list.unwrap_err(),
            mock_invalid_chapter_list.unwrap_err().to_string()
        );
    }
}
