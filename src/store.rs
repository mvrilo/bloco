use crate::blob::Blob;
use crate::Result;

pub trait Store: Default + Clone {
    fn get(&mut self, hash: [u8; 32]) -> Option<Blob>;
    fn put(&mut self, data: Vec<u8>) -> Result<[u8; 32]>;
}
