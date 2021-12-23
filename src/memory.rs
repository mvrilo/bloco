use crate::blob::{Blob, Hash};
use crate::store::Store;
use crate::Result;

pub type BlobMap = std::collections::BTreeMap<Hash, Blob>;

#[derive(Debug, Clone, Default)]
pub struct MemoryStore {
    db: BlobMap,
}

impl Store for MemoryStore {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        self.db.get(&hash).map(|blob| blob.clone())
    }

    fn put(&mut self, blob: Blob) -> Result<()> {
        let db = &mut self.db;
        db.insert(blob.hash, blob);
        Ok(())
    }
}
