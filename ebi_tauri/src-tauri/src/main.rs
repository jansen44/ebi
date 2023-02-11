#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ebi::source::{EbiSource, Manga, SourceLoader};
use ebi::SourceManager;
use tauri::State;

#[tauri::command]
fn sources(source_manager: State<SourceManager>) -> Vec<EbiSource> {
    let sources = source_manager.available_sources();

    sources
        .iter()
        .map(|src| source_manager.get(src).unwrap().source().unwrap())
        .collect()
}

#[tauri::command]
fn manga_list(identifier: &str, source_manager: State<SourceManager>) -> Option<Vec<Manga>> {
    let source = source_manager.get(identifier)?;
    let manga = source.manga_list();

    match manga {
        Ok(manga) => Some(manga),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn main() {
    let home = std::env::var("HOME").unwrap(); // TODO: handle this error and "Windows" later

    let mut source_manager = SourceManager::new(&format!("{home}/.ebi/sources"));
    source_manager.load_source("opex").unwrap();

    tauri::Builder::default()
        .manage(source_manager)
        .invoke_handler(tauri::generate_handler![sources, manga_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
