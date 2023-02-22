use ebi_source::SourceLoader;
use std::{collections::HashMap, path::PathBuf};

use crate::error::EbiError;

use super::loader::Source;
use super::{EbiChapter, EbiManga, EbiSource};

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

#[cfg(target_family = "unix")]
impl std::default::Default for SourceManager {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();
        let path = PathBuf::from(format!("{}/.ebi", home));
        std::fs::create_dir_all(&path).unwrap();

        Self {
            dir_path: path,
            sources: HashMap::new(),
        }
    }
}

// Resource loading
impl SourceManager {
    pub fn sources(&self) -> Vec<EbiSource> {
        self.sources
            .iter()
            .map(|(_, source)| source.source().unwrap())
            .collect()
    }

    pub fn manga_list(&self, source: &str) -> Result<Vec<EbiManga>, EbiError> {
        let source = self.sources.get(source).ok_or(EbiError::InvalidSource)?;
        source.manga_list()
    }

    pub fn chapter_list(&self, manga: &EbiManga) -> Result<Vec<EbiChapter>, EbiError> {
        let source = self
            .sources
            .get(&manga.source_identifier.clone())
            .ok_or(EbiError::InvalidSource)?;
        source.chapter_list(manga)
    }

    pub fn chapter_page_list(&self, chapter: &EbiChapter) -> Result<Vec<String>, EbiError> {
        let source = self
            .sources
            .get(&chapter.source_identifier)
            .ok_or(EbiError::InvalidSource)?;
        source.chapter_page_list(chapter)
    }
}

// Initialization/Setup
impl SourceManager {
    pub fn new(dir_path: &str) -> Self {
        let dir_path = PathBuf::from(dir_path);

        Self {
            dir_path,
            sources: HashMap::new(),
        }
    }

    pub fn dir(&self) -> PathBuf {
        self.dir_path.clone()
    }

    // <self.dir>/sources
    pub fn source_dir(&self) -> PathBuf {
        let mut source_dir = self.dir_path.clone();
        source_dir.push("sources");
        source_dir
    }

    // TODO: Refactor this later (probably after adding "install source" support)
    pub fn load_sources(&mut self) -> Result<(), EbiError> {
        let source_dir_entries = std::fs::read_dir(self.source_dir()).unwrap();

        let source_dir_directories = source_dir_entries
            .filter(|d| d.is_ok())
            .map(|d| d.unwrap())
            .filter(|d| d.metadata().is_ok())
            .filter(|d| d.metadata().unwrap().is_dir());

        for dir in source_dir_directories {
            let identifier = dir.file_name();
            let identifier = identifier.to_str();

            if let None = identifier {
                log::warn!(
                    "Could not load source file at {} :: invalid os file_name",
                    dir.path().display()
                );
                continue;
            }

            let target_file = handle_source_file_extension(identifier.unwrap());

            let mut file_path = dir.path();
            file_path.push(target_file);

            if !file_path.exists() {
                log::warn!(
                    "Could not load source file {} :: file does not exists",
                    file_path.display()
                );
                continue;
            }

            match Source::try_from(file_path) {
                Ok(source) => {
                    let identifier = source.source()?.identifier.clone();
                    log::info!("Loaded source {}", &identifier);
                    self.sources.insert(identifier, source);
                }
                Err(e) => {
                    println!("ERROR::{}", e);
                }
            }
        }

        Ok(())
    }
}
