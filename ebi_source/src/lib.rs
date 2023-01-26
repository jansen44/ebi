use serde::{Deserialize, Serialize};

#[allow(dead_code)]
mod abi;

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

#[async_trait::async_trait]
pub trait SourceLoader {
    type Error;

    fn source(&self) -> Result<Source, Self::Error>;

    async fn manga_list(&self) -> Result<Vec<Manga>, Self::Error>;
    async fn latest_manga(&self) -> Result<Vec<Manga>, Self::Error>;
    async fn popular_manga(&self) -> Result<Vec<Manga>, Self::Error>;
    async fn hot_manga(&self) -> Result<Vec<Manga>, Self::Error>;

    async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>, Self::Error>;
    async fn get_manga(&self, manga_identifier: &str) -> Result<Manga, Self::Error>;

    async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>, Self::Error>;
    async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>, Self::Error>;

    async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>, Self::Error>;
}
