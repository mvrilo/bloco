use crate::{lru::LRUStore, store::Store};

#[derive(Debug, Clone)]
pub struct Bloco<B: Store, const CACHE_SIZE: usize> {
    pub blobstore: B,
    pub cache: LRUStore<CACHE_SIZE>,
}

impl<B, const CACHE_SIZE: usize> Bloco<B, CACHE_SIZE>
where
    B: Store,
{
    pub fn new(blobstore: B) -> Self {
        let cache = LRUStore::<CACHE_SIZE>::default();
        Bloco { blobstore, cache }
    }
}
