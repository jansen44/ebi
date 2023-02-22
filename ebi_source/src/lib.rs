use serde::{Deserialize, Serialize};

pub mod abi;
pub mod error;
pub mod locale;
pub mod prelude;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chapter {
    pub chapter: u16,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
    pub source_identifier: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
    pub genres: Vec<String>,
    pub description: Option<String>,
    pub source_identifier: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub locale: locale::Locale,
}

pub trait SourceLoader {
    type Error;

    fn source(&self) -> Result<Source, Self::Error>;

    fn manga_list(&self) -> Result<Vec<Manga>, Self::Error>;
    fn chapter_list(&self, manga: &Manga) -> Result<Vec<Chapter>, Self::Error>;
    fn chapter_page_list(&self, chapter: &Chapter) -> Result<Vec<String>, Self::Error>;
}
