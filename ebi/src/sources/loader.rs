use std::path::PathBuf;

use ebi_source::{
    abi::{
        chapter::{
            chapter_list::{ABIChapterListInput, ChapterListFn},
            chapter_page_list::{ABIChapterPageListInput, ChapterPageListFn},
        },
        manga::manga_list::MangaListFn,
        source::source_info::SourceInfoFn,
    },
    error::SourceError,
};
use libloading::Library;

use crate::error::EbiError;

use super::{EbiChapter, EbiManga, EbiSource, SourceLoader};

pub struct Source {
    lib: Library,
    source: EbiSource,
}

impl std::convert::TryFrom<PathBuf> for Source {
    type Error = EbiError;

    fn try_from(source_path: PathBuf) -> Result<Self, Self::Error> {
        let lib = unsafe { Library::new(source_path.clone()).map_err(|_| EbiError::LoadLib)? };
        log::debug!("Loaded Source from {}", source_path.display());

        let source_fn = unsafe {
            lib.get::<SourceInfoFn>(b"abi_source_info")
                .map_err(|_| EbiError::LoadFunction)?
        };

        let source: Result<EbiSource, SourceError> = source_fn().into();
        let source = source.map_err(|e| {
            log::error!("Error loading source: {}", e);
            EbiError::SourceError(e.to_string())
        })?;

        Ok(Self { lib, source })
    }
}

impl SourceLoader for Source {
    type Error = EbiError;

    fn source_info(&self) -> Result<EbiSource, Self::Error> {
        Ok(self.source.clone())
    }

    fn manga_list(&self) -> Result<Vec<EbiManga>, Self::Error> {
        let manga_list = unsafe {
            self.lib
                .get::<MangaListFn>(b"abi_manga_list")
                .map_err(|_| EbiError::LoadFunction)?
        };
        let manga_list: Result<Vec<EbiManga>, SourceError> = manga_list().into();
        let manga_list = manga_list.map_err(|e| {
            log::error!("Error manga list: {}", e);
            EbiError::SourceError(e.to_string())
        })?;

        Ok(manga_list)
    }

    fn chapter_list(&self, manga: &EbiManga) -> Result<Vec<EbiChapter>, Self::Error> {
        let manga = ABIChapterListInput::from(manga);

        let chapter_list = unsafe {
            self.lib
                .get::<ChapterListFn>(b"abi_chapter_list")
                .map_err(|_| EbiError::LoadFunction)?
        };
        let chapter_list: Result<Vec<EbiChapter>, SourceError> = chapter_list(manga).into();
        let chapter_list = chapter_list.map_err(|e| {
            log::error!("Error chapter list: {}", e);
            EbiError::SourceError(e.to_string())
        })?;

        Ok(chapter_list)
    }

    fn chapter_page_list(&self, chapter: &EbiChapter) -> Result<Vec<String>, Self::Error> {
        let chapter = ABIChapterPageListInput::from(chapter);
        let chapter_page_list = unsafe {
            self.lib
                .get::<ChapterPageListFn>(b"abi_chapter_page_list")
                .map_err(|_| EbiError::LoadFunction)?
        };
        let chapter_page_list: Result<Vec<String>, SourceError> = chapter_page_list(chapter).into();
        let chapter_page_list = chapter_page_list.map_err(|e| {
            log::error!("Error chapter page list: {}", e);
            EbiError::SourceError(e.to_string())
        })?;

        Ok(chapter_page_list)
    }
}
