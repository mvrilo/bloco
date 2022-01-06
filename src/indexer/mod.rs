use crate::{Chunk, FileRef, Hash, Result};
use async_trait::async_trait;

pub mod sqlite;

pub use crate::indexer::sqlite::SqliteIndexer;

#[async_trait]
pub trait FileRefIndexer: Sync + Send + Clone {
    async fn put(&self, fr: &FileRef) -> Result<()>;
    async fn get_by_name(&self, name: String) -> Result<Vec<FileRef>>;
    async fn get_by_hash(&self, hash: Hash) -> Result<Vec<FileRef>>;
}

#[async_trait]
pub trait ChunkIndexer: Sync + Send + Clone {
    async fn put_chunk(&self, origin: Hash, chunk: Hash) -> Result<()>;
    async fn get_chunks(&self, origin: Hash) -> Result<Vec<Chunk>>;
}
