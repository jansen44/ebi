pub mod error;

mod source;

use ebi_source::SourceLoader;
use error::EbiError;
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
