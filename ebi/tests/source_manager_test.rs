fn get_chapters(
    source_identifier: &str,
    manga: &ebi_source::Manga,
    size: u16,
) -> Vec<ebi_source::Chapter> {
    (1..size + 1)
        .map(|chapter| ebi_source::Chapter {
            chapter,
            title: format!("{} -- {}", manga.title.clone(), chapter),
            url: format!("{}/{}", manga.url.clone(), chapter),
            manga_identifier: manga.identifier.clone(),
            source_identifier: source_identifier.to_string(),
        })
        .collect()
}

mod valid_tests {
    use ebi::SourceManager;
    use ebi_source::{locale, prelude::SourceError, Chapter, Manga, Source, SourceLoader};

    const SOURCE_DIR: &str = "../target/debug";

    const SOURCE_IDENTIFIER: &str = "valid_source_macro_mock";
    const SOURCE_TITLE: &str = "Mocked Valid Ebi Extension";
    const SOURCE_DESCRIPTION: &str =
        "This is just a mocked source only intended to be used for tests! No real content here";

    pub fn source() -> Source {
        Source {
            identifier: SOURCE_IDENTIFIER.to_owned(),
            title: SOURCE_TITLE.to_owned(),
            description: SOURCE_DESCRIPTION.to_owned(),
            locale: locale::Locale::EnUs,
        }
    }

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

    pub fn chapter_list(manga: Manga) -> Result<Vec<Chapter>, SourceError> {
        Ok(super::get_chapters(SOURCE_IDENTIFIER, &manga, 100))
    }

    #[test]
    fn load_valid_source() {
        let mock_source = source();

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&mock_source.identifier).unwrap();

        let dy_source = source_manager
            .get(&mock_source.identifier)
            .unwrap()
            .source()
            .unwrap();
        assert_eq!(dy_source.identifier, mock_source.identifier);
        assert_eq!(dy_source.title, mock_source.title);
        assert_eq!(dy_source.description, mock_source.description);
        assert_eq!(dy_source.locale, mock_source.locale);

        assert_eq!(
            source_manager.available_sources(),
            vec![dy_source.identifier]
        );
    }

    #[test]
    fn load_valid_manga_list() {
        let mock_source = source();
        let mock_manga_list = manga_list().unwrap();

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&mock_source.identifier).unwrap();

        let dy_manga_list = source_manager
            .get(&mock_source.identifier)
            .unwrap()
            .manga_list()
            .unwrap();
        assert_eq!(mock_manga_list.len(), dy_manga_list.len());

        for (mock, dy) in mock_manga_list.iter().zip(dy_manga_list.iter()) {
            assert_eq!(mock.identifier, dy.identifier);
            assert_eq!(mock.title, dy.title);
            assert_eq!(mock.url, dy.url);
            assert_eq!(mock.cover, dy.cover);
            assert_eq!(mock.genres, dy.genres);
            assert_eq!(mock.description, dy.description);
            assert_eq!(mock.source_identifier, dy.source_identifier);
        }
    }

    #[test]
    fn load_valid_chapter_list() {
        let mock_source = source();
        let mock_manga_list = manga_list().unwrap();

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&mock_source.identifier).unwrap();

        let dy_source = source_manager.get(&mock_source.identifier).unwrap();

        for manga in mock_manga_list.iter() {
            let mock_chapter_list = chapter_list(manga.clone()).unwrap();
            let dy_chapter_list = dy_source.chapter_list(manga.clone()).unwrap();
            assert_eq!(mock_chapter_list.len(), dy_chapter_list.len());

            for (mock, dy) in mock_chapter_list.iter().zip(dy_chapter_list.iter()) {
                assert_eq!(mock.chapter, dy.chapter);
                assert_eq!(mock.title, dy.title);
                assert_eq!(mock.url, dy.url);
                assert_eq!(mock.manga_identifier, dy.manga_identifier);
                assert_eq!(mock.source_identifier, dy.source_identifier);
            }
        }
    }
}

mod broken_tests {
    use ebi::SourceManager;
    use ebi_source::{error::SourceError, locale, Chapter, Manga, Source, SourceLoader};

    const SOURCE_DIR: &str = "../target/debug";

    const SOURCE_IDENTIFIER: &str = "invalid_source_macro_mock";
    const SOURCE_TITLE: &str = "Mocked Invalid Ebi Extension";
    const SOURCE_DESCRIPTION: &str =
        "This is just a mocked source only intended to be used for tests! No real content here";

    pub fn source() -> Source {
        Source {
            identifier: SOURCE_IDENTIFIER.to_owned(),
            title: SOURCE_TITLE.to_owned(),
            description: SOURCE_DESCRIPTION.to_owned(),
            locale: locale::Locale::EnUs,
        }
    }

    pub fn chapter_list(manga: Manga) -> Result<Vec<Chapter>, SourceError> {
        if &manga.identifier == valid_manga().identifier.as_str() {
            return Ok(super::get_chapters(SOURCE_IDENTIFIER, &manga, 100));
        }

        Err(SourceError::Unknown(format!(
            "It was not possible to load chapters for \"{}\"",
            manga.identifier
        )))
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

    #[test]
    fn cant_load_invalid_source() {
        let invalid_source = "invalid_source_identifier";

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        assert!(source_manager.load_source(invalid_source).is_err());
    }

    #[test]
    fn cant_add_same_valid_source_multiple_times() {
        let source = source();

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        assert!(source_manager.load_source(&source.identifier).is_ok());
        assert!(source_manager.load_source(&source.identifier).is_err());
        assert!(source_manager.load_source(&source.identifier).is_err());
    }

    #[test]
    fn cant_load_manga_list() {
        let source = source();

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&source.identifier).unwrap();
        let source = source_manager.get(&source.identifier).unwrap();

        let err_manga_list = source.manga_list();

        assert!(err_manga_list.is_err());
    }

    #[test]
    fn cant_load_chapter_list() {
        let source = source();
        let manga = valid_manga();
        let invalid_manga = invalid_manga();

        let mock_chapter_list = chapter_list(manga.clone()).unwrap();

        let mut source_manager = SourceManager::new(SOURCE_DIR);
        source_manager.load_source(&source.identifier).unwrap();
        let source = source_manager.get(&source.identifier).unwrap();

        let valid_chapter_list = source.chapter_list(manga.clone()).unwrap();

        for (mock, dy) in mock_chapter_list.iter().zip(valid_chapter_list.iter()) {
            assert_eq!(mock.chapter, dy.chapter);
            assert_eq!(mock.title, dy.title);
            assert_eq!(mock.url, dy.url);
            assert_eq!(mock.manga_identifier, dy.manga_identifier);
            assert_eq!(mock.source_identifier, dy.source_identifier);
        }

        let err_chapter_list = source.chapter_list(invalid_manga);
        assert!(err_chapter_list.is_err());
    }
}
