use std::convert::From;

use crate::{error::SourceError, Chapter};

use super::primitives::FFIString;

#[repr(C)]
pub struct ABIChapter {
    pub chapter: u32,
    pub title: FFIString,
    pub url: FFIString,
    pub manga: FFIString,
    pub source: FFIString,
}

impl From<&Chapter> for ABIChapter {
    fn from(chapter: &Chapter) -> Self {
        let title = FFIString::from(chapter.title.clone());
        let url = FFIString::from(chapter.url.clone());
        let manga = FFIString::from(chapter.manga.clone());
        let source = FFIString::from(chapter.source.clone());

        Self {
            chapter: chapter.chapter,
            title,
            url,
            manga,
            source,
        }
    }
}

impl TryInto<Chapter> for ABIChapter {
    type Error = SourceError;

    fn try_into(self) -> Result<Chapter, Self::Error> {
        let title = self.title.try_into()?;
        let url = self.url.try_into()?;
        let manga = self.manga.try_into()?;
        let source = self.source.try_into()?;

        Ok(Chapter {
            chapter: self.chapter,
            title,
            url,
            manga,
            source,
        })
    }
}

pub mod chapter_list {
    use std::convert::From;

    use crate::abi::primitives::{ABIResultArray, FFIString};
    use crate::Manga;

    pub type ChapterListFn = extern "C" fn(ABIChapterListInput) -> ABIResultArray;

    #[repr(C)]
    pub struct ABIChapterListInput {
        pub identifier: FFIString,
        pub url: FFIString,
    }

    impl From<&Manga> for ABIChapterListInput {
        fn from(manga: &Manga) -> Self {
            Self {
                identifier: FFIString::from(manga.identifier.clone()),
                url: FFIString::from(manga.url.clone()),
            }
        }
    }
}

pub mod chapter_page_list {
    use std::convert::From;

    use crate::abi::primitives::{ABIResultArray, FFIString};
    use crate::Chapter;

    pub type ChapterPageListFn = extern "C" fn(ABIChapterPageListInput) -> ABIResultArray;

    #[repr(C)]
    pub struct ABIChapterPageListInput {
        pub chapter: u32,
        pub chapter_url: FFIString,
        pub manga: FFIString,
    }

    impl From<&Chapter> for ABIChapterPageListInput {
        fn from(chapter: &Chapter) -> Self {
            ABIChapterPageListInput {
                chapter: chapter.chapter,
                chapter_url: FFIString::from(chapter.url.clone()),
                manga: FFIString::from(chapter.manga.clone()),
            }
        }
    }
}
