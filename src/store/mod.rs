use async_trait::async_trait;

pub mod cached;
pub mod chunk;
pub mod encrypted;
pub mod file;
pub mod lru;

use crate::{Blob, Hash, Result};

#[async_trait]
pub trait Store: Sync + Send + Clone {
    async fn get(&mut self, hash: Hash) -> Result<Blob>;
    async fn put(&mut self, blob: &mut Blob) -> Result<Hash>;
}
