pub mod blob;
pub mod bloco;
pub mod error;
pub mod lru;
pub mod memory;
pub mod store;

use blob::Blob;
use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, error::Error>;

pub type BlobMap = BTreeMap<[u8; 32], Blob>;
