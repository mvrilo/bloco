use crate::blob::{Blob, Hash};
use crate::Result;
use crate::{file::FileStore, lru::LRUStore, store::Store};

#[derive(Debug, Clone)]
pub struct Bloco<'a, const CACHE_SIZE: usize> {
    pub root: FileStore<'a>,
    pub cache: LRUStore<CACHE_SIZE>,
}

impl<'a, const CACHE_SIZE: usize> Bloco<'a, CACHE_SIZE> {
    pub fn new(root: &'a str) -> Self {
        let root = FileStore::new(root);
        let cache = LRUStore::<CACHE_SIZE>::default();
        Bloco { root, cache }
    }
}

impl<'a, const CACHE_SIZE: usize> Store for Bloco<'a, CACHE_SIZE> {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        match self.cache.get(hash) {
            Some(data) => Some(data),
            None => self.root.get(hash),
        }
    }

    fn put(&mut self, data: Blob) -> Result<()> {
        let blob: Blob = data.into();
        self.root.put(blob.clone())?;
        self.cache.put(blob)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_store() {
        let mut bloco = Bloco::<100>::new("/tmp/bloco-cargo-test");
        let file_a: Blob = b"hello".to_vec().into();
        bloco.put(file_a.clone()).unwrap();
        let blob_a = bloco.get(file_a.hash).unwrap();
        assert_eq!(file_a.hash, blob_a.hash);
    }
}
