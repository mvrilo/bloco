use crate::LRUStore;
use crate::{Blob, Hash, Result, Store};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct CachedStore<S: Store, const N: usize> {
    cache: LRUStore<N>,
    store: S,
}

impl<S, const N: usize> CachedStore<S, N>
where
    S: Store,
{
    pub fn new(store: S) -> Self {
        CachedStore {
            cache: LRUStore::default(),
            store,
        }
    }
}

#[async_trait]
impl<S, const N: usize> Store for CachedStore<S, N>
where
    S: Store,
{
    async fn get(&mut self, hash: Hash) -> Result<Blob> {
        self.cache.get(hash).await.or(self.store.get(hash).await)
    }

    async fn put(&mut self, blob: &mut Blob) -> Result<Hash> {
        self.store.put(blob).await?;
        self.cache.put(blob).await?;
        Ok(blob.hash())
    }
}
