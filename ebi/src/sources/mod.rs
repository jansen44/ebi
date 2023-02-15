mod loader;
pub mod manager;

pub use ebi_source::SourceLoader;
pub use ebi_source::{Chapter as EbiChapter, Manga as EbiManga, Source as EbiSource};

pub use manager::SourceManager;
