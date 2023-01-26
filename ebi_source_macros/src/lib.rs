use proc_macro::TokenStream;

mod abi;

#[proc_macro_attribute]
pub fn ebi_plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    abi::gen_ebi_plugin(args, input)
}

// #[async_trait::async_trait]
// pub trait Source {
//     fn identifier(&self) -> Cow<str>;
//     fn title(&self) -> Cow<str>;
//     fn description(&self) -> Cow<str>;
//     fn locale(&self) -> locale::Locale;

//     async fn manga_list(&self) -> Result<Vec<Manga>>;
//     async fn latest_manga(&self) -> Result<Vec<Manga>>;
//     async fn popular_manga(&self) -> Result<Vec<Manga>>;
//     async fn hot_manga(&self) -> Result<Vec<Manga>>;

//     async fn search_manga(&self, manga_title: &str) -> Result<Vec<Manga>>;
//     async fn get_manga(&self, manga_identifier: &str) -> Result<Manga>;

//     async fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>>;
//     async fn chapter(&self, manga: Manga, chapter: usize) -> Result<Option<Chapter>>;

//     async fn page_url_list(&self, chapter: Chapter) -> Result<Vec<String>>;
// }
