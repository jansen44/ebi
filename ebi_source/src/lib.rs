pub mod abi;
pub mod locale;

#[derive(Clone, Debug)]
pub struct Chapter {
    pub chapter: u16,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
    pub source_identifier: String,
}

#[derive(Clone, Debug)]
pub struct Manga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
    pub genres: Vec<String>,
    pub description: Option<String>,
    pub source_identifier: String,
}

#[derive(Clone, Debug)]
pub struct Source {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub locale: locale::Locale,
}
