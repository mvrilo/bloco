use crate::LRUStore;
use crate::{Blob, Hash, Result, Store};

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

impl<S, const N: usize> Store for CachedStore<S, N>
where
    S: Store,
{
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        self.cache.get(hash).or_else(|| self.store.get(hash))
    }

    fn put(&mut self, blob: &mut Blob) -> Result<()> {
        self.store.put(blob)?;
        self.cache.put(blob)?;
        Ok(())
    }
}
