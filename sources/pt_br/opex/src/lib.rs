use std::borrow::Cow;

use ebi_source::{Chapter, Manga, Result, Source as EbiSource};

const OPEX_SOURCE_IDENTIFIER: &str = "opex";
const OPEX_SOURCE_TITLE: &str = "One Piece Ex";
const OPEX_SOURCE_DESCRIPTION: &str = "One Piece Ex | De fã para fã";
const _OPEX_BASE_URL: &str = "https://onepieceex.net";

pub struct Source;

#[async_trait::async_trait]
impl EbiSource for Source {
    fn identifier(&self) -> Cow<str> {
        Cow::Borrowed(OPEX_SOURCE_IDENTIFIER)
    }

    fn title(&self) -> Cow<str> {
        Cow::Borrowed(OPEX_SOURCE_TITLE)
    }

    fn description(&self) -> Cow<str> {
        Cow::Borrowed(OPEX_SOURCE_DESCRIPTION)
    }

    async fn manga_list(&self) -> Result<Vec<Manga>> {
        let main = Manga {
            identifier: String::from("main"),
            title: String::from("One Piece"),
            cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg"),
            url: String::from("/mangas"),
            genre: vec![String::from("shounen"), String::from("fantasy")],
            description: None,
            source_identifier: OPEX_SOURCE_IDENTIFIER.to_string(),
        };

        let cover = Manga {
            identifier: String::from("covers"),
            title: String::from("One Piece - Histórias de Capa"),
            cover: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
            url: String::from("/historias-de-capa"),
            genre: vec![String::from("shounen"), String::from("fantasy")],
            description: None,
            source_identifier: OPEX_SOURCE_IDENTIFIER.to_string(),
        };

        let sbs = Manga {
            identifier: String::from("sbs"),
            title: String::from("One Piece - SBS"),
            cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg"),
            url: String::from("/sbs"),
            genre: vec![String::from("shounen"), String::from("fantasy")],
            description: None,
            source_identifier: OPEX_SOURCE_IDENTIFIER.to_string(),
        };

        Ok(vec![main.into(), cover.into(), sbs.into()])
    }

    async fn latest_manga(&self) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn popular_manga(&self) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn hot_manga(&self) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>> {
        todo!()
    }
    async fn get_manga(&self, manga_identifier: &str) -> Result<Manga> {
        todo!()
    }

    async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>> {
        todo!()
    }
    async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>> {
        todo!()
    }

    async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>> {
        todo!()
    }
}

#[no_mangle]
pub fn register() -> Box<dyn EbiSource> {
    let s = Source {};
    Box::new(s)
}
