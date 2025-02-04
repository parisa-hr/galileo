use crate::error::GalileoError;
use crate::layer::data_provider::PersistentCacheController;
use bytes::Bytes;
use log::debug;
use std::path::{Path, PathBuf};

const CACHE_FOLDER: &str = ".tile_cache";

#[derive(Debug, Clone)]
pub struct FileCacheController {
    folder_path: PathBuf,
}

impl Default for FileCacheController {
    fn default() -> Self {
        Self::new(CACHE_FOLDER)
    }
}

impl PersistentCacheController<str, Bytes> for FileCacheController {
    fn get(&self, key: &str) -> Option<Bytes> {
        let file_path = self.get_file_path(key);
        if let Ok(bytes) = std::fs::read(file_path) {
            Some(bytes.into())
        } else {
            None
        }
    }

    fn insert(&self, key: &str, data: &Bytes) -> Result<(), GalileoError> {
        let file_path = self.get_file_path(key);
        ensure_folder_exists(file_path.parent().unwrap()).unwrap();
        std::fs::write(&file_path, data)?;

        debug!("Entry {key} saved to cache file {file_path:?}");

        Ok(())
    }
}

impl FileCacheController {
    pub fn new(path: impl AsRef<Path>) -> Self {
        ensure_folder_exists(path.as_ref()).unwrap();
        Self {
            folder_path: path.as_ref().into(),
        }
    }

    fn get_file_path(&self, url: &str) -> PathBuf {
        let stripped = if let Some(v) = url.strip_prefix("http://") {
            v
        } else if let Some(v) = url.strip_prefix("https://") {
            v
        } else {
            url
        };

        self.folder_path.join(Path::new(stripped))
    }
}

fn ensure_folder_exists(folder_path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(folder_path)
}
