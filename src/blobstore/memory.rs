use crate::blob::Blob;
use crate::blobstore::{BlobMap, BlobStore};
use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct MemoryBlobStore {
    db: BlobMap,
}

impl BlobStore for MemoryBlobStore {
    fn get(&mut self, hash: [u8; 32]) -> Option<Blob> {
        self.db.get(&hash).map(|blob| blob.clone())
    }

    fn put(&mut self, data: Vec<u8>) -> Result<[u8; 32]> {
        let hash: [u8; 32] = blake3::hash(&data).into();
        let db = &mut self.db;
        db.insert(hash, data.into());
        Ok(hash)
    }
}
