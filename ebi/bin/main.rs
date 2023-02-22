use ebi::sources::{archive::SourceArchiver, SourceManager};

fn main() {
    let mut sources = SourceManager::default();
    let archive = SourceArchiver::from(&sources);
    sources.load_sources().unwrap();

    let manga = sources.manga_list("opex").unwrap();
    let manga = manga.get(0).unwrap();
    let manga = archive.save_manga_cover(manga).unwrap();

    let chapters = sources.chapter_list(&manga).unwrap();
    let chapter = &chapters[chapters.len() - 1];

    let pages = sources.chapter_page_list(chapter).unwrap();
    for page in pages.iter().enumerate() {
        let (page, url) = page;
        let cached = archive
            .save_chapter_pages(chapter, (url, page as u32))
            .unwrap();
        println!("{page} -- {url} :: {cached}");
    }
}
