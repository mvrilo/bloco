use crate::blob::{Blob, Hash};
use crate::Result;

pub trait Store: Clone {
    fn get(&mut self, hash: Hash) -> Option<Blob>;
    fn put(&mut self, blob: Blob) -> Result<()>;
}
