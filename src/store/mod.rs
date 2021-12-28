pub mod btree;
pub mod file;
pub mod lru;

use crate::{Blob, Hash, Result};

pub trait Store: Clone {
    fn get(&mut self, hash: Hash) -> Option<Blob>;
    fn put(&mut self, blob: Blob) -> Result<()>;
}
