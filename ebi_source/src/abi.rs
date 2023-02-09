use std::ffi::{c_char, CString};

use crate::error::SourceError;
use crate::{Chapter, Manga};

pub type JSONResourceFn = extern "C" fn() -> *mut c_char;
pub type JSONInputedResourceFn<T> = extern "C" fn(input: T) -> *mut c_char;

#[repr(C)]
pub struct ABIChapterListInput {
    pub manga_identifier: *mut c_char,
    pub manga_url: *mut c_char,
}

impl std::convert::TryFrom<Manga> for ABIChapterListInput {
    type Error = SourceError;

    fn try_from(manga: Manga) -> Result<Self, Self::Error> {
        let manga_identifier = CString::new(manga.identifier)?;
        let manga_url = CString::new(manga.url)?;

        let manga_identifier = manga_identifier.into_raw();
        let manga_url = manga_url.into_raw();

        Ok(Self {
            manga_identifier,
            manga_url,
        })
    }
}

#[repr(C)]
pub struct ABIChapterPageListInput {
    pub chapter: u16,
    pub chapter_url: *mut c_char,
    pub manga_identifier: *mut c_char,
}

impl std::convert::TryFrom<Chapter> for ABIChapterPageListInput {
    type Error = SourceError;
    fn try_from(chapter: Chapter) -> Result<Self, Self::Error> {
        let chapter_url = CString::new(chapter.url)?;
        let manga_identifier = CString::new(chapter.manga_identifier)?;

        let chapter_url = chapter_url.into_raw();
        let manga_identifier = manga_identifier.into_raw();

        Ok(ABIChapterPageListInput {
            chapter: chapter.chapter,
            chapter_url,
            manga_identifier,
        })
    }
}
