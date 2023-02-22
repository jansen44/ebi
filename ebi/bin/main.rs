// use ebi::download::http_download;

use ebi::sources::SourceManager;

fn main() {
    let mut sources = SourceManager::default();
    sources.load_sources().unwrap();

    // let manga = sources.manga_list("opex").unwrap();

    // for m in manga.iter() {
    //     println!("{:?}", m);
    // }

    // let chapters = sources.chapter_list(&manga[0]).unwrap();
    // for chapter in chapters.iter() {
    //     println!("{chapter:?}");
    // }

    // let pages = sources
    //     .chapter_page_list(&chapters[chapters.len() - 1])
    //     .unwrap();
    // for page in pages {
    //     println!("{page}");
    // }
}
