use ebi_source::{Chapter, SourceLoader};
use std::{collections::HashMap, path::PathBuf};

use crate::archive;
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

    pub fn sources(&self) -> Vec<EbiSource> {
        self.sources
            .iter()
            .map(|(_, source)| source.source().unwrap())
            .collect()
    }

    pub fn manga_list(&self, source: &str) -> Result<Vec<EbiManga>, EbiError> {
        let source = self.sources.get(source).ok_or(EbiError::InvalidSource)?;
        let manga_list = source.manga_list()?;

        Ok(manga_list
            .into_iter()
            .map(|manga| self.init_manga(manga))
            .collect())
    }

    pub fn chapter_list(&self, manga: &EbiManga) -> Result<Vec<Chapter>, EbiError> {
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
        let pages = source.chapter_page_list(chapter)?;
        Ok(Self::init_chapter_pages(&self, chapter, pages))
    }

    fn init_manga(&self, manga: EbiManga) -> EbiManga {
        let mut dir_path = self.dir_path.clone();
        dir_path.push(format!(
            "sources/{}/manga/{}",
            &manga.source_identifier, &manga.identifier
        ));
        std::fs::create_dir_all(&dir_path).unwrap();

        let mut manga = manga;
        let mut cached_logo = false;

        for d in std::fs::read_dir(dir_path.clone()).unwrap() {
            let f = d.unwrap();
            let f_name = f.file_name().to_str().unwrap().to_owned();
            let mut f_path = dir_path.clone();

            if f_name.contains("logo") {
                f_path.push(f_name);
                manga.cover = f_path.to_string_lossy().into_owned();
                cached_logo = true;
                break;
            }
        }

        if !cached_logo {
            let file_ext = archive::http_download(&manga.cover, "logo", dir_path.clone()).unwrap();
            let mut f_path = dir_path.clone();
            f_path.push(format!("logo.{}", file_ext));
            manga.cover = f_path.to_string_lossy().into_owned();
        }

        manga
    }

    fn init_chapter_pages(&self, chapter: &EbiChapter, pages: Vec<String>) -> Vec<String> {
        let mut dir_path = self.dir_path.clone();
        dir_path.push(format!(
            "sources/{}/manga/{}/{}",
            &chapter.source_identifier, &chapter.manga_identifier, chapter.chapter
        ));
        std::fs::create_dir_all(&dir_path).unwrap();

        let mut cached_pages = vec![];
        for cached_page in std::fs::read_dir(dir_path.clone()).unwrap() {
            let cached_page = cached_page.unwrap();
            let ext = cached_page.path();
            let ext = ext.extension().unwrap();
            let ext = ext.to_string_lossy().into_owned();

            if &ext == "jpg" || &ext == "jpeg" || &ext == "png" {
                cached_pages.push(cached_page.path().to_string_lossy().into_owned());
            }
        }

        let mut pages = pages;
        for (i, page) in pages.iter_mut().enumerate() {
            let cached = cached_pages
                .iter()
                .find(|cp| cp.contains(&format!("/{i}.")));

            if let Some(cached_page) = cached {
                *page = cached_page.clone();
                continue;
            }
            let file_ext =
                archive::http_download(&page, &format!("{i}"), dir_path.clone()).unwrap();
            let mut f_path = dir_path.clone();
            f_path.push(format!("{i}.{}", file_ext));
            *page = f_path.to_string_lossy().into_owned();
        }
        pages
    }
}
