use ebi_source::abi::manga::{ABIManga, ABIMangaList};
use ebi_source::abi::source::ABISource;
use ebi_source::{locale, Manga, Source};

const SOURCE_IDENTIFIER: &str = "opex";
const SOURCE_TITLE: &str = "One Piece Ex";
const SOURCE_DESCRIPTION: &str = "One Piece Ex | De fã para fã";
const _BASE_URL: &str = "https://onepieceex.net";

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

#[no_mangle]
extern "C" fn manga_list() -> ABIMangaList {
    let main = Manga {
        identifier: String::from("main"),
        title: String::from("One Piece"),
        cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg"),
        url: String::from("/mangas"),
        genres: vec![String::from("shounen"), String::from("fantasy")],
        description: None,
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    };

    let cover = Manga {
        identifier: String::from("covers"),
        title: String::from("One Piece - Histórias de Capa"),
        cover: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
        url: String::from("/historias-de-capa"),
        genres: vec![String::from("shounen"), String::from("fantasy")],
        description: None,
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    };

    let sbs = Manga {
        identifier: String::from("sbs"),
        title: String::from("One Piece - SBS"),
        cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg"),
        url: String::from("/sbs"),
        genres: vec![String::from("shounen"), String::from("fantasy")],
        description: None,
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    };

    ABIMangaList::from_vec(vec![main.into(), cover.into(), sbs.into()])
}

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
