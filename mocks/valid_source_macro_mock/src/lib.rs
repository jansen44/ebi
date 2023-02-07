use ebi_source::error::SourceError;
use ebi_source::prelude::*;
use ebi_source::{locale, Chapter, Manga, Source};
use ebi_source_macros::ebi_plugin;

const SOURCE_IDENTIFIER: &str = "valid_source_macro_mock";
const SOURCE_TITLE: &str = "Mocked Valid Ebi Extension";
const SOURCE_DESCRIPTION: &str =
    "This is just a mocked source only intended to be used for tests! No real content here";

#[ebi_plugin]
pub fn source() -> Result<Source, SourceError> {
    Ok(Source {
        identifier: SOURCE_IDENTIFIER.to_owned(),
        title: SOURCE_TITLE.to_owned(),
        description: SOURCE_DESCRIPTION.to_owned(),
        locale: locale::Locale::EnUs,
    })
}

#[ebi_plugin]
pub fn manga_list() -> Result<Vec<Manga>, SourceError> {
    let manga = vec![Manga {
        identifier: "one-piece".to_string(),
        title: "One Piece".to_string(),
        cover: "http://127.0.0.1/fake-cover/one-piece".to_string(),
        genres: vec!["shounen".to_string(), "fantasy".to_string()],
        description: Some("Rubber pirate boy adventures".to_string()),
        url: "/manga/one-piece".to_string(),
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    }];
    Ok(manga)
}

#[ebi_plugin]
pub fn chapter_list(
    manga_identifier: String,
    manga_url: String,
) -> Result<Vec<Chapter>, SourceError> {
    Ok(get_chapters(&manga_identifier, &manga_url, 100))
}

fn get_chapters(identifier: &str, url: &str, size: u16) -> Vec<Chapter> {
    let manga = manga_list().unwrap();
    let manga = manga.iter().find(|m| m.identifier == identifier).unwrap();
    (1..size + 1)
        .map(|chapter| Chapter {
            chapter,
            title: format!("{} -- {}", &manga.title, chapter),
            url: format!("{}/{}", url, chapter),
            manga_identifier: identifier.to_string(),
            source_identifier: SOURCE_IDENTIFIER.to_string(),
        })
        .collect()
}
