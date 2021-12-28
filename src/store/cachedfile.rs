use crate::{Blob, Hash, Result, Store};
use crate::{FileStore, LRUStore};

#[derive(Debug, Clone, Default)]
pub struct CachedFileStore<const N: usize> {
    cache: LRUStore<N>,
    fs: FileStore,
}

impl<const N: usize> CachedFileStore<N> {
    pub fn new(dir: String) -> Self {
        CachedFileStore {
            cache: LRUStore::default(),
            fs: FileStore::new(dir),
        }
    }
}

impl<const N: usize> Store for CachedFileStore<N> {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        self.cache.get(hash).or_else(|| self.fs.get(hash))
    }

    fn put(&mut self, blob: &mut Blob) -> Result<()> {
        self.fs.put(blob)?;
        self.cache.put(blob)?;
        Ok(())
    }
}
