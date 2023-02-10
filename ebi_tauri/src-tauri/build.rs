use std::fs;

fn main() {
    fs::create_dir_all("../build").unwrap();
    tauri_build::build()
}
