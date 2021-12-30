use crate::{Hash, Result};
use bincode::{config::Configuration, Decode, Encode};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Configuration = Configuration::standard();
}

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct FileRef {
    pub name: String,
    pub size: u64,
    pub orig_hash: Hash,
    pub hashes: Vec<Hash>,
}

impl FileRef {
    pub fn new(name: String, size: u64, orig_hash: Hash, hashes: Vec<Hash>) -> Self {
        FileRef {
            name,
            hashes,
            orig_hash,
            size,
        }
    }

    pub fn from_vec(arr: &[u8]) -> Result<Vec<FileRef>> {
        let (rref, _): (Vec<FileRef>, usize) = bincode::decode_from_slice(arr, *CONFIG)?;
        Ok(rref)
    }

    pub fn from_slice(arr: &[u8]) -> Result<FileRef> {
        let (rref, _): (FileRef, usize) = bincode::decode_from_slice(arr, *CONFIG)?;
        Ok(rref)
    }
}
