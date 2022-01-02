use crate::{Blob, Hash, Result, Store};
use async_trait::async_trait;
use uluru::LRUCache;

#[derive(Debug, Clone, Default)]
pub struct LRUStore<const N: usize> {
    db: LRUCache<Blob, N>,
}

#[async_trait]
impl<const N: usize> Store for LRUStore<N> {
    async fn get(&mut self, hash: Hash) -> Result<Blob> {
        self.db
            .find(|x| x.hash() == hash)
            .map(|blob| blob.clone())
            .ok_or(crate::error::Error::NotFound)
    }

    async fn put(&mut self, blob: &mut Blob) -> Result<Hash> {
        let db = &mut self.db;
        db.insert(blob.clone());
        Ok(blob.hash())
    }
}
