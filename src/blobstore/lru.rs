use crate::blob::Blob;
use crate::blobstore::BlobStore;
use crate::Result;
use uluru::LRUCache;

#[derive(Debug, Clone, Default)]
pub struct LRUBlobStore<const N: usize> {
    db: LRUCache<Blob, N>,
}

impl<const N: usize> BlobStore for LRUBlobStore<N> {
    fn get(&mut self, hash: [u8; 32]) -> Option<Blob> {
        self.db.find(|x| x.hash() == hash).map(|blob| blob.clone())
    }

    fn put(&mut self, data: Vec<u8>) -> Result<[u8; 32]> {
        let hash: [u8; 32] = blake3::hash(&data).into();
        let db = &mut self.db;
        db.insert(data.into());
        Ok(hash)
    }
}
