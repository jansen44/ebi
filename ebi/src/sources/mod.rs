pub(crate) mod loader;
pub mod manager;

pub use ebi_source::SourceLoader;
pub use ebi_source::{Chapter as EbiChapter, Manga as EbiManga, Source as EbiSource};

pub use manager::SourceManager;

pub(crate) fn ptr_to_string(ptr: *mut i8) -> String {
    let string = unsafe { std::ffi::CString::from_raw(ptr) };
    string.to_string_lossy().to_string()
}
