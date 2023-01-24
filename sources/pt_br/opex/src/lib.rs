// use ebi_source::{locale, Chapter, Manga, Result, Source as EbiSource};
use ebi_source::{locale, Source};

use std::ffi::CString;

const SOURCE_IDENTIFIER: &str = "opex";
const SOURCE_TITLE: &str = "One Piece Ex";
const SOURCE_DESCRIPTION: &str = "One Piece Ex | De fã para fã";
const _BASE_URL: &str = "https://onepieceex.net";

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

// #[no_mangle]
// fn locale() -> *const i8 {
//     locale::Locale::PtBr.to_string().as_ptr()
// }

// async fn manga_list(&self) -> Result<Vec<Manga>> {
//     let main = Manga {
//         identifier: String::from("main"),
//         title: String::from("One Piece"),
//         cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg"),
//         url: String::from("/mangas"),
//         genre: vec![String::from("shounen"), String::from("fantasy")],
//         description: None,
//         source_identifier: SOURCE_IDENTIFIER.to_string(),
//     };

//     let cover = Manga {
//         identifier: String::from("covers"),
//         title: String::from("One Piece - Histórias de Capa"),
//         cover: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
//         url: String::from("/historias-de-capa"),
//         genre: vec![String::from("shounen"), String::from("fantasy")],
//         description: None,
//         source_identifier: SOURCE_IDENTIFIER.to_string(),
//     };

//     let sbs = Manga {
//         identifier: String::from("sbs"),
//         title: String::from("One Piece - SBS"),
//         cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg"),
//         url: String::from("/sbs"),
//         genre: vec![String::from("shounen"), String::from("fantasy")],
//         description: None,
//         source_identifier: SOURCE_IDENTIFIER.to_string(),
//     };

//     Ok(vec![main.into(), cover.into(), sbs.into()])
// }

// async fn latest_manga(&self) -> Result<Vec<Manga>> {
//     self.manga_list().await
// }

// async fn popular_manga(&self) -> Result<Vec<Manga>> {
//     self.manga_list().await
// }

// async fn hot_manga(&self) -> Result<Vec<Manga>> {
//     self.manga_list().await
// }

// async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>> {
//     todo!()
// }
// async fn get_manga(&self, manga_identifier: &str) -> Result<Manga> {
//     todo!()
// }

// async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>> {
//     todo!()
// }
// async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>> {
//     todo!()
// }

// async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>> {
//     todo!()
// }
