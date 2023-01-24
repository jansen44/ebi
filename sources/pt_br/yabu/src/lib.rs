use ebi_source::abi::source::ABISource;
use ebi_source::{locale, Source};

const SOURCE_IDENTIFIER: &str = "yabu";
const SOURCE_TITLE: &str = "Manga Yabu";
const SOURCE_DESCRIPTION: &str = "Manga Yabu! - Ler Mangás Online";
const _BASE_URL: &str = "https://mangayabu.top";

#[no_mangle]
extern "C" fn source() -> ABISource {
    let source = Source {
        identifier: SOURCE_IDENTIFIER.to_owned(),
        title: SOURCE_TITLE.to_owned(),
        description: SOURCE_DESCRIPTION.to_owned(),
        locale: locale::Locale::PtBr,
    };
    source.into()
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
