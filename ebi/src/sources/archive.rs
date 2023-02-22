use std::{fs::DirEntry, path::PathBuf};

use crate::{downloader::http_download, error::EbiError};

use super::{EbiChapter, EbiManga, SourceManager};

const MANGA_COVER_FILE_NAME: &str = "cover";

pub struct SourceArchiver {
    dir_path: PathBuf,
}

impl std::convert::From<&SourceManager> for SourceArchiver {
    fn from(value: &SourceManager) -> Self {
        Self {
            dir_path: value.source_dir(),
        }
    }
}

impl SourceArchiver {
    pub fn save_manga_cover(&self, manga: &EbiManga) -> Result<EbiManga, EbiError> {
        let manga_path = self.manga_path(&manga.source_identifier, &manga.identifier);
        std::fs::create_dir_all(&manga_path)?;

        let find_patt = format!("/{MANGA_COVER_FILE_NAME}.");
        let saved_cover = std::fs::read_dir(&manga_path)?.find_map(Self::find_map_file(&find_patt));

        let mut manga = manga.clone();
        manga.cover = self.download_if_not_exists(
            saved_cover,
            &manga.cover,
            MANGA_COVER_FILE_NAME,
            manga_path,
        )?;

        Ok(manga)
    }

    pub fn save_chapter_pages(
        &self,
        chapter: &EbiChapter,
        page: (&str, u32),
    ) -> Result<String, EbiError> {
        let chapter_path = self.chapter_path(chapter);
        std::fs::create_dir_all(&chapter_path).unwrap();

        let (page_url, page_number) = page;

        let find_patt = format!("/{page_number}.");
        let saved_page =
            std::fs::read_dir(&chapter_path)?.find_map(Self::find_map_file(&find_patt));

        self.download_if_not_exists(
            saved_page,
            &page_url,
            &format!("{page_number}"),
            chapter_path,
        )
    }
}

impl SourceArchiver {
    fn base_manga_path_from_source(&self, source: &str) -> PathBuf {
        let mut manga_path = self.dir_path.clone();
        manga_path.push(format!("{}/manga/", source));
        manga_path
    }

    fn manga_path(&self, source: &str, manga: &str) -> PathBuf {
        let mut manga_path = self.base_manga_path_from_source(source);
        manga_path.push(manga);
        manga_path
    }

    fn chapter_path(&self, chapter: &EbiChapter) -> PathBuf {
        let mut chapter_path =
            self.manga_path(&chapter.source_identifier, &chapter.manga_identifier);
        chapter_path.push(format!("{}", chapter.chapter));
        chapter_path
    }

    fn download_if_not_exists(
        &self,
        cached: Option<PathBuf>,
        url: &str,
        file_name: &str,
        dir_path: PathBuf,
    ) -> Result<String, EbiError> {
        match cached {
            Some(cached_path) => Ok(cached_path.to_string_lossy().into_owned()),
            None => {
                let file_ext = http_download(&url, file_name, &dir_path)?;

                let mut f_path = dir_path;
                f_path.push(format!("{file_name}.{file_ext}"));

                Ok(f_path.to_string_lossy().into_owned())
            }
        }
    }

    fn find_map_file(
        pattern: &str,
    ) -> Box<dyn FnMut(Result<DirEntry, std::io::Error>) -> Option<PathBuf>> {
        let pattern = pattern.to_owned().clone();
        Box::new(move |file| {
            let file = file.ok()?;
            let file_name = file.path();
            let file_name = file_name.to_str()?;

            if file_name.contains(&pattern) {
                return Some(file.path());
            }
            None
        })
    }
}
