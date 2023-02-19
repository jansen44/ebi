// use ebi::download::http_download;

use ebi::sources::SourceManager;

fn main() {
    let mut sources = SourceManager::default();
    sources.load_local_sources().unwrap();

    let manga = sources.manga_list("opex").unwrap();

    for m in manga {
        println!("{:?}", m);
        // let chapters = sources.manga_list("opex").unwrap().chapter_list(&m).unwrap();
        // let chapter_url = &chapters[0].url;

        // let
    }

    // let chapters = sources
    //     .get("yabu")
    //     .unwrap()
    //     .chapter_list(&manga[0])
    //     .unwrap();
    // println!("{:?}", chapters);
}
