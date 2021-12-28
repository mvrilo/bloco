pub mod blob;
pub mod bloco;
pub mod core;
pub mod error;
pub mod hash;
pub mod indexer;
pub mod store;

pub use crate::blob::Blob;
pub use crate::blob::Ref;
pub use crate::bloco::{Bloco, Default};
pub use crate::core::Core;
pub use crate::error::Error;
pub use crate::hash::Hash;

pub use crate::indexer::Indexer;
pub use crate::indexer::SledIndexer;

pub use crate::store::btree::BTreeStore;
pub use crate::store::cachedfile::CachedFileStore;
pub use crate::store::encrypted::EncryptedStore;
pub use crate::store::file::FileStore;
pub use crate::store::lru::LRUStore;
pub use crate::store::Store;

pub type Result<T> = std::result::Result<T, error::Error>;
