use ebi_source::error::SourceError;
use ebi_source::prelude::*;
use ebi_source::{locale, Chapter, Manga, Source};
use ebi_source_macros::ebi_plugin;

const SOURCE_IDENTIFIER: &str = "invalid_source_macro_mock";
const SOURCE_TITLE: &str = "Mocked Invalid Ebi Extension";
const SOURCE_DESCRIPTION: &str =
    "This is just a mocked source only intended to be used for tests! No real content here";

#[ebi_plugin]
pub fn source() -> Source {
    Source {
        identifier: SOURCE_IDENTIFIER.to_owned(),
        title: SOURCE_TITLE.to_owned(),
        description: SOURCE_DESCRIPTION.to_owned(),
        locale: locale::Locale::EnUs,
    }
}

#[ebi_plugin]
pub fn manga_list() -> Result<Vec<Manga>, SourceError> {
    Err(SourceError::Fetch)
}

#[ebi_plugin]
pub fn chapter_list(
    manga_identifier: String,
    manga_url: String,
) -> Result<Vec<Chapter>, SourceError> {
    if &manga_identifier != valid_manga().identifier.as_str() {
        return Err(SourceError::Unknown(format!(
            "It was not possible to load chapters for \"{}\"",
            manga_identifier
        )));
    }

    Ok(get_chapters(&manga_identifier, &manga_url, 100))
}

pub fn valid_manga() -> Manga {
    Manga {
        identifier: "valid".to_string(),
        cover: "http://127.0.0.1/valid-manga-cover".to_string(),
        description: None,
        genres: vec![],
        source_identifier: SOURCE_IDENTIFIER.to_string(),
        title: "A Valid Manga Title".to_string(),
        url: "http://127.0.0.1/valid-manga".to_string(),
    }
}

pub fn invalid_manga() -> Manga {
    Manga {
        identifier: "invalid".to_string(),
        cover: "http://127.0.0.1/invalid-manga-cover".to_string(),
        description: None,
        genres: vec![],
        source_identifier: SOURCE_IDENTIFIER.to_string(),
        title: "A Invalid Manga Title".to_string(),
        url: "http://127.0.0.1/invalid-manga".to_string(),
    }
}

fn get_chapters(manga_identifier: &str, manga_url: &str, size: u16) -> Vec<Chapter> {
    let manga = match manga_identifier {
        "valid" => valid_manga(),
        _ => invalid_manga(),
    };

    (1..size + 1)
        .map(|chapter| Chapter {
            chapter,
            title: format!("{} -- {}", manga.title, chapter),
            url: format!("{}/{}", manga_url, chapter),
            manga_identifier: manga_identifier.to_string(),
            source_identifier: SOURCE_IDENTIFIER.to_string(),
        })
        .collect()
}
