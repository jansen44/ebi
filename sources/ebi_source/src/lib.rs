use std::borrow::Cow;

pub const REGISTER_FUNCTION_NAME: &[u8] = b"register";

#[macro_export]
macro_rules! register_source {
    () => {
        // Unique source alias
        use ebi_source::Source as EbiMacroSource;

        #[no_mangle]
        pub fn register() -> Box<dyn EbiMacroSource> {
            let s = Source {};
            Box::new(s)
        }
    };
}

// TODO: Remove anyhow and use custom result
pub use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Chapter {
    pub chapter: u16,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
    pub source_identifier: String,
}

unsafe impl Sync for Chapter {}
unsafe impl Send for Chapter {}

#[derive(Clone, Debug)]
pub struct Manga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
    pub genre: Vec<String>,
    pub description: Option<String>,
    pub source_identifier: String,
}

unsafe impl Sync for Manga {}
unsafe impl Send for Manga {}

#[async_trait::async_trait]
pub trait Source {
    fn identifier(&self) -> Cow<str>;
    fn title(&self) -> Cow<str>;
    fn description(&self) -> Cow<str>;

    async fn manga_list(&self) -> Result<Vec<Manga>>;
    async fn latest_manga(&self) -> Result<Vec<Manga>>;
    async fn popular_manga(&self) -> Result<Vec<Manga>>;
    async fn hot_manga(&self) -> Result<Vec<Manga>>;

    async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>>;
    async fn get_manga(&self, manga_identifier: &str) -> Result<Manga>;

    async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>>;
    async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>>;

    async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>>;
}
