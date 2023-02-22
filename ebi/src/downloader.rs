use std::path::{Path, PathBuf};

use crate::error::EbiError;

pub fn http_download<P>(
    url: &str,
    file_name: &str,
    target_dir: P,
) -> Result<KnownFileExtensions, EbiError>
where
    P: AsRef<Path>,
{
    let mut path: PathBuf = target_dir.as_ref().into();
    if path.is_file() {
        return Err(EbiError::InvalidDir(path.to_string_lossy().into_owned()));
    }

    let mut buffer = Vec::new();
    let response = ureq::get(url).call()?;

    let content_type = response
        .header("content-type")
        .ok_or(EbiError::NoContentType)?;
    let file_ext = KnownFileExtensions::try_from_content_type(content_type)?;
    path.push(format!("{}.{}", file_name, file_ext));

    response
        .into_reader()
        .read_to_end(&mut buffer)
        .map_err(|_| EbiError::CouldNotReadBuffer)?;

    std::fs::write(path, buffer).map_err(|e| EbiError::CouldNotSaveFile(e.to_string()))?;

    Ok(file_ext)
}

pub enum KnownFileExtensions {
    Jpeg,
    Png,
}

impl KnownFileExtensions {
    pub fn try_from_content_type(header: &str) -> Result<Self, EbiError> {
        match header {
            "image/jpeg" => Ok(Self::Jpeg),
            "image/png" => Ok(Self::Png),
            _ => Err(EbiError::UnsupportedFile(String::from(header))),
        }
    }
}

impl std::fmt::Display for KnownFileExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Jpeg => "jpg",
                Self::Png => "png",
            }
        )
    }
}
