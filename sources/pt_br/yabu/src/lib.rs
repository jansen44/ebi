use ebi_source::{locale, Source};

use std::ffi::CString;
// use std::borrow::Cow;

// use ebi_source::{locale, Chapter, Manga, Result, Source as EbiSource};

const SOURCE_IDENTIFIER: &str = "yabu";
const SOURCE_TITLE: &str = "Manga Yabu";
const SOURCE_DESCRIPTION: &str = "Manga Yabu! - Ler Mangás Online";
const _BASE_URL: &str = "https://mangayabu.top";

#[no_mangle]
extern "C" fn source() -> Source {
    let identifier = CString::new(SOURCE_IDENTIFIER).unwrap();
    let title = CString::new(SOURCE_TITLE).unwrap();
    let description = CString::new(SOURCE_DESCRIPTION).unwrap();

    Source {
        identifier: identifier.into_raw(),
        title: title.into_raw(),
        description: description.into_raw(),
        locale: locale::Locale::PtBr as u32,
    }
}

// pub struct Source;

// #[async_trait::async_trait]
// impl EbiSource for Source {
//     fn identifier(&self) -> Cow<str> {
//         Cow::Borrowed(SOURCE_IDENTIFIER)
//     }

//     fn title(&self) -> Cow<str> {
//         Cow::Borrowed(SOURCE_TITLE)
//     }

//     fn description(&self) -> Cow<str> {
//         Cow::Borrowed(SOURCE_DESCRIPTION)
//     }

//     fn locale(&self) -> locale::Locale {
//         locale::Locale::PtBr
//     }

//     async fn manga_list(&self) -> Result<Vec<Manga>> {
//         todo!()
//     }

//     async fn latest_manga(&self) -> Result<Vec<Manga>> {
//         todo!()
//     }

//     async fn popular_manga(&self) -> Result<Vec<Manga>> {
//         todo!()
//     }

//     async fn hot_manga(&self) -> Result<Vec<Manga>> {
//         todo!()
//     }

//     async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>> {
//         todo!()
//     }
//     async fn get_manga(&self, manga_identifier: &str) -> Result<Manga> {
//         todo!()
//     }

//     async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>> {
//         todo!()
//     }
//     async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>> {
//         todo!()
//     }

//     async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>> {
//         todo!()
//     }
// }

// ebi_source::register_source!();
