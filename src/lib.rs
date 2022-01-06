pub mod blob;
pub mod bloco;
pub mod chunk;
pub mod core;
pub mod error;
pub mod fileref;
pub mod hash;
pub mod indexer;
pub mod store;

pub use crate::blob::Blob;
pub use crate::bloco::Bloco;
pub use crate::chunk::Chunk;
pub use crate::core::Core;
pub use crate::error::Error;
pub use crate::fileref::FileRef;
pub use crate::hash::Hash;

pub use crate::indexer::ChunkIndexer;
pub use crate::indexer::FileRefIndexer;
pub use crate::indexer::SqliteIndexer;

pub use crate::store::cached::CachedStore;
pub use crate::store::chunk::ChunkStore;
pub use crate::store::encrypted::EncryptedStore;
pub use crate::store::file::FileStore;
pub use crate::store::lru::LRUStore;
pub use crate::store::Store;

pub type Result<T> = std::result::Result<T, error::Error>;

pub type DefaultStore<const N: usize> =
    Bloco<EncryptedStore<CachedStore<ChunkStore<FileStore, SqliteIndexer>, N>>, SqliteIndexer>;
