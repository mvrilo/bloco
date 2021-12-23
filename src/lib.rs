pub mod blob;
pub mod bloco;
pub mod error;
pub mod file;
pub mod lru;
pub mod memory;
pub mod store;

pub use crate::blob::Blob;
pub use crate::bloco::Bloco;
pub use crate::error::Error;
pub use crate::file::FileStore;
pub use crate::lru::LRUStore;
pub use crate::memory::MemoryStore;
pub use crate::store::Store;

pub type Result<T> = std::result::Result<T, error::Error>;
