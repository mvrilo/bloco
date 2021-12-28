use crate::{Blob, Hash, Result, Store};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct BTreeStore {
    db: BTreeMap<Hash, Blob>,
}

impl Store for BTreeStore {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        self.db.get(&hash).cloned()
    }

    fn put(&mut self, blob: Blob) -> Result<()> {
        let db = &mut self.db;
        db.insert(blob.hash(), blob);
        Ok(())
    }
}
