use crate::{Blob, Hash, Result, Store};
use async_trait::async_trait;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct FileStore {
    dir: String,
}

impl FileStore {
    pub fn new(dir: String) -> Self {
        std::fs::create_dir_all(&dir).unwrap();
        FileStore { dir }
    }
}

#[async_trait]
impl Store for FileStore {
    async fn get(&mut self, hash: Hash) -> Result<Blob> {
        let name = String::from_utf8_lossy(&hash.0).into_owned();
        let path = Path::new(&self.dir).join(&name);
        Ok(fs::File::open(path)?.into())
    }

    async fn put(&mut self, blob: &mut Blob) -> Result<Hash> {
        let hash = blob.hash();
        let path = Path::new(&self.dir).join(hash.as_hex());
        let mut file = fs::File::create(path)?;
        file.write_all(&blob.0)?;
        Ok(hash)
    }
}
