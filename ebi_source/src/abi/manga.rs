use crate::{error::SourceError, Manga};

use super::primitives::{FFIArray, FFIString};

#[repr(C)]
pub struct ABIManga {
    pub identifier: FFIString,
    pub title: FFIString,
    pub cover: FFIString,
    pub url: FFIString,
    pub genres: FFIArray,
    pub description: FFIString,
    pub source: FFIString,
}

impl From<&Manga> for ABIManga {
    fn from(manga: &Manga) -> Self {
        let description = match manga.description {
            Some(ref description) => FFIString::from(description.clone()),
            None => FFIString::null(),
        };

        let genres = FFIArray::from(manga.genres.clone());
        let identifier = FFIString::from(manga.identifier.clone());
        let title = FFIString::from(manga.title.clone());
        let cover = FFIString::from(manga.cover.clone());
        let url = FFIString::from(manga.url.clone());
        let source = FFIString::from(manga.source.clone());

        Self {
            description,
            genres,
            identifier,
            title,
            cover,
            url,
            source,
        }
    }
}

impl TryInto<Manga> for ABIManga {
    type Error = SourceError;

    fn try_into(self) -> Result<Manga, Self::Error> {
        let description = match self.description.is_null() {
            true => None,
            false => Some(self.description.try_into()?),
        };

        let genres = self.genres.try_into()?;
        let identifier = self.identifier.try_into()?;
        let title = self.title.try_into()?;
        let cover = self.cover.try_into()?;
        let url = self.url.try_into()?;
        let source = self.source.try_into()?;

        Ok(Manga {
            description,
            genres,
            identifier,
            title,
            cover,
            url,
            source,
        })
    }
}

pub mod manga_list {
    use crate::abi::primitives::ABIResultArray;

    pub type MangaListFn = extern "C" fn() -> ABIResultArray;
}
