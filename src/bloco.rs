use crate::blob::{Blob, Hash};
use crate::Result;
use crate::{lru::LRUStore, store::Store};

#[derive(Debug, Clone)]
pub struct Bloco<B: Store, const CACHE_SIZE: usize> {
    pub store: B,
    pub cache: LRUStore<CACHE_SIZE>,
}

impl<B, const CACHE_SIZE: usize> Bloco<B, CACHE_SIZE>
where
    B: Store,
{
    pub fn new(store: B) -> Self {
        let cache = LRUStore::<CACHE_SIZE>::default();
        Bloco { store, cache }
    }
}

impl<B, const CACHE_SIZE: usize> Store for Bloco<B, CACHE_SIZE>
where
    B: Store,
{
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        match self.cache.get(hash) {
            Some(data) => Some(data),
            None => self.store.get(hash),
        }
    }

    fn put(&mut self, data: Blob) -> Result<()> {
        let blob: Blob = data.into();
        self.store.put(blob.clone())?;
        self.cache.put(blob)?;
        Ok(())
    }
}
