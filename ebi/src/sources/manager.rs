use ebi_source::SourceLoader;
use std::{collections::HashMap, path::PathBuf};

use crate::error::EbiError;

use super::loader::Source;

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

impl SourceManager {
    pub fn new(dir_path: &str) -> Self {
        let dir_path = PathBuf::from(dir_path);

        Self {
            dir_path,
            sources: HashMap::new(),
        }
    }

    pub fn load_local_sources(&mut self) -> Result<(), EbiError> {
        let mut sources_dir = self.dir_path.clone();
        sources_dir.push("sources");

        let source_dirs = std::fs::read_dir(&sources_dir).unwrap();

        let source_dirs = source_dirs
            .filter(|d| d.is_ok())
            .map(|d| d.unwrap())
            .filter(|d| d.metadata().is_ok())
            .filter(|d| d.metadata().unwrap().is_dir());

        for dir in source_dirs {
            let identifier = dir.file_name();
            let identifier = identifier.to_string_lossy().into_owned();
            let target_file = handle_source_file_extension(&identifier);

            let mut file_path = dir.path();
            file_path.push(target_file);

            if !file_path.exists() {
                println!("File {} does not exists", file_path.display());
                continue;
            }

            match Source::try_from(file_path) {
                Ok(source) => {
                    let identifier = source.source()?.identifier.clone();
                    self.sources.insert(identifier, source);
                }
                Err(e) => {
                    println!("ERROR::{}", e);
                }
            }
        }

        Ok(())
    }

    pub fn load_source(&mut self, identifier: &str) -> Result<(), EbiError> {
        if self.sources.contains_key(identifier) {
            return Err(EbiError::DuplicatedSource);
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
