use std::ffi::CString;
use std::path::PathBuf;

use libloading::Library;

use ebi_source::abi::{ABIChapterListInput, JSONInputedResourceFn, JSONResourceFn};
use ebi_source::prelude::{serde_json, Deserialize, SourceErrorSerialized};
pub use ebi_source::{Chapter, Manga, Source as EbiSource, SourceLoader};

use crate::error::EbiError;

fn ptr_to_string(ptr: *mut i8) -> String {
    let string = unsafe { CString::from_raw(ptr) };
    string.to_string_lossy().to_string()
}

macro_rules! json_fn_to_string {
    ($f:expr) => {{
        let response = $f();
        ptr_to_string(response)
    }};
    ($f:expr, $input:expr) => {{
        let response = $f($input);
        ptr_to_string(response)
    }};
}

fn json_serialize<'a, T: Deserialize<'a>>(response: &'a str) -> Result<T, EbiError> {
    let serialized = serde_json::from_str(&response);
    match serialized {
        Ok(serialized) => Ok(serialized),
        Err(_) => {
            let err: SourceErrorSerialized =
                serde_json::from_str(&response).map_err(|_| EbiError::SerializeResponse)?;
            Err(EbiError::LoadChapterList(err))
        }
    }
}

pub struct Source {
    lib: Library,
    source: EbiSource,
}

impl Source {
    fn get_abi_func_response(&self, name: &str) -> Result<String, EbiError> {
        let f = unsafe { self.lib.get::<JSONResourceFn>(name.as_bytes()) };
        match f {
            Ok(f) => Ok(json_fn_to_string!(f)),
            Err(_) => Err(EbiError::LoadFunction),
        }
    }

    fn get_abi_func_with_input_response<T>(
        &self,
        name: &str,
        input: T,
    ) -> Result<String, EbiError> {
        let f = unsafe { self.lib.get::<JSONInputedResourceFn<T>>(name.as_bytes()) };
        match f {
            Ok(f) => Ok(json_fn_to_string!(f, input)),
            Err(_) => Err(EbiError::LoadFunction),
        }
    }
}

// TODO: Add logs
impl std::convert::TryFrom<PathBuf> for Source {
    type Error = EbiError;

    fn try_from(source_path: PathBuf) -> Result<Self, Self::Error> {
        let lib = unsafe { Library::new(source_path).map_err(|_| EbiError::LoadLib)? };

        let source_fn = unsafe {
            lib.get::<JSONResourceFn>(b"abi_source")
                .map_err(|_| EbiError::LoadFunction)?
        };

        let source = json_fn_to_string!(source_fn);
        let source = serde_json::from_str(&source);

        match source {
            Ok(source) => Ok(Source { lib, source }),
            Err(_) => Err(EbiError::SerializeResponse),
        }
    }
}

impl SourceLoader for Source {
    type Error = EbiError;

    fn source(&self) -> Result<EbiSource, Self::Error> {
        Ok(self.source.clone())
    }

    fn manga_list(&self) -> Result<Vec<Manga>, Self::Error> {
        let manga_list = self.get_abi_func_response("abi_manga_list")?;
        json_serialize(&manga_list)
    }

    fn chapter_list(&self, manga: Manga) -> Result<Vec<Chapter>, Self::Error> {
        let manga = ABIChapterListInput::try_from(manga)?;
        let chapter_list = self.get_abi_func_with_input_response("abi_chapter_list", manga)?;
        json_serialize(&chapter_list)
    }
}
