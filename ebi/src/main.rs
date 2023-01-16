use ebi_source::Source;

fn load_plugin() -> Box<dyn Source> {
    unsafe {
        let p = libloading::Library::new("./target/debug/libopex.dylib").unwrap();
        let register_plugin: libloading::Symbol<unsafe extern "C" fn() -> Box<dyn Source>> =
            p.get(b"register").unwrap();

        return register_plugin();
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("HELLO");
    let opex = load_plugin();

    println!(
        "{} -- {} -- {}",
        opex.identifier(),
        opex.title(),
        opex.description()
    );

    let manga = opex.manga_list().await.unwrap();
    for m in manga.iter() {
        println!("{} == {}", m.identifier, m.title);
    }
    Ok(())
}
