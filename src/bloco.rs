use crate::blobstore::{lru::LRUBlobStore, BlobStore};

#[derive(Debug, Clone)]
pub struct Bloco<B: BlobStore, const CACHE_SIZE: usize> {
    pub blobstore: B,
    pub cache: LRUBlobStore<CACHE_SIZE>,
}

impl<B, const CACHE_SIZE: usize> Bloco<B, CACHE_SIZE>
where
    B: BlobStore,
{
    pub fn new(blobstore: B) -> Self {
        let cache = LRUBlobStore::<CACHE_SIZE>::default();
        Bloco { blobstore, cache }
    }
}
