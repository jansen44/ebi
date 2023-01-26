use std::ffi::{c_char, CString};
use std::ptr::null_mut;
use std::slice;

use crate::Manga;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ABIManga {
    pub identifier: *mut c_char,
    pub title: *mut c_char,
    pub cover: *mut c_char,
    pub url: *mut c_char,

    pub genre: *const *mut c_char,
    pub genre_len: usize,

    pub description: *mut c_char,

    pub source_identifier: *mut c_char,
}

impl std::convert::Into<Manga> for ABIManga {
    fn into(self) -> Manga {
        let identifier = unsafe { CString::from_raw(self.identifier) };
        let title = unsafe { CString::from_raw(self.title) };
        let cover = unsafe { CString::from_raw(self.cover) };
        let url = unsafe { CString::from_raw(self.url) };
        let source_identifier = unsafe { CString::from_raw(self.source_identifier) };

        let identifier = identifier.to_string_lossy().into_owned();
        let title = title.to_string_lossy().into_owned();
        let cover = cover.to_string_lossy().into_owned();
        let url = url.to_string_lossy().into_owned();
        let source_identifier = source_identifier.to_string_lossy().into_owned();

        let description = if self.description.is_null() {
            None
        } else {
            let description = unsafe { CString::from_raw(self.description) };
            let description = description.to_string_lossy().into_owned();
            Some(description)
        };

        let genres = unsafe {
            let genres = slice::from_raw_parts(self.genre, self.genre_len)
                .into_iter()
                .map(|g| CString::from_raw(*g))
                .map(|g| g.to_string_lossy().into_owned())
                .collect();
            std::mem::drop(*self.genre);
            genres
        };

        Manga {
            identifier,
            title,
            cover,
            url,
            source_identifier,
            description,
            genres,
        }
    }
}

impl std::convert::From<Manga> for ABIManga {
    fn from(value: Manga) -> Self {
        let identifier = CString::new(value.identifier).unwrap();
        let title = CString::new(value.title).unwrap();
        let cover = CString::new(value.cover).unwrap();
        let url = CString::new(value.url).unwrap();
        let source_identifier = CString::new(value.source_identifier).unwrap();

        let description = match value.description {
            Some(desc) => CString::new(desc).unwrap().into_raw(),
            None => null_mut(),
        };

        let genre_len = value.genres.len();
        let genre: Vec<*mut i8> = value
            .genres
            .clone()
            .iter()
            .map(|g| CString::new(g.as_str()).unwrap().into_raw())
            .collect();

        let manga = Self {
            identifier: identifier.into_raw(),
            title: title.into_raw(),
            cover: cover.into_raw(),
            url: url.into_raw(),
            source_identifier: source_identifier.into_raw(),

            description,

            genre_len,
            genre: genre.as_ptr(),
        };

        std::mem::forget(genre);

        return manga;
    }
}

#[repr(C)]
pub struct ABIMangaList {
    pub manga: *const ABIManga,
    pub len: usize,
}

impl ABIMangaList {
    pub fn from_vec(manga: Vec<Manga>) -> Self {
        let manga_list: Vec<ABIManga> = manga.clone().into_iter().map(|m| m.into()).collect();

        let manga = ABIMangaList {
            len: manga_list.len(),
            manga: manga_list.as_ptr(),
        };
        std::mem::forget(manga_list);
        manga
    }

    pub fn into_vec(self) -> Vec<Manga> {
        let manga = unsafe {
            let manga = slice::from_raw_parts(self.manga, self.len)
                .into_iter()
                .map(|m| (*m).into())
                .collect();
            std::mem::drop(*self.manga);
            manga
        };
        manga
    }
}