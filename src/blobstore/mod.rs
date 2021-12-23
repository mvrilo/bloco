use crate::blob::Blob;
use crate::Result;
use std::collections::BTreeMap;

pub mod lru;
pub mod memory;

pub trait BlobStore: Default + Clone {
    fn get(&mut self, hash: [u8; 32]) -> Option<Blob>;
    fn put(&mut self, data: Vec<u8>) -> Result<[u8; 32]>;
}

pub type BlobMap = BTreeMap<[u8; 32], Blob>;
