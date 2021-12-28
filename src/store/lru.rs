use crate::{Blob, Hash, Result, Store};
use uluru::LRUCache;

#[derive(Debug, Clone, Default)]
pub struct LRUStore<const N: usize> {
    db: LRUCache<Blob, N>,
}

impl<const N: usize> Store for LRUStore<N> {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        self.db.find(|x| x.hash() == hash).map(|blob| blob.clone())
    }

    fn put(&mut self, blob: &mut Blob) -> Result<()> {
        let db = &mut self.db;
        db.insert(blob.clone());
        Ok(())
    }
}
