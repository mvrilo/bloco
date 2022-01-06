use crate::{Blob, Hash, Result, Store};
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

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
        let path = Path::new(&self.dir).join(hash.as_hex());
        Ok(Blob::read_file(fs::File::open(path).await?).await?)
    }

    async fn put(&mut self, blob: &mut Blob) -> Result<Hash> {
        let hash = blob.hash();
        let path = Path::new(&self.dir).join(hash.as_hex());
        let mut file = fs::File::create(path).await?;
        file.write_all(&blob.0).await?;
        file.sync_all().await?;
        Ok(hash)
    }
}
