use crate::Hash;
use bincode::{Decode, Encode};
use std::fs;
use std::io::Read;

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Blob(pub Vec<u8>);

impl Blob {
    pub fn new(data: Vec<u8>) -> Self {
        Blob(data)
    }

    pub fn hash(&self) -> Hash {
        blake3::hash(&self.0).into()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {
        Blob::new(data)
    }
}

impl From<&[u8]> for Blob {
    fn from(arr: &[u8]) -> Blob {
        Blob::new(arr.into())
    }
}

impl From<fs::File> for Blob {
    fn from(mut file: fs::File) -> Blob {
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        Blob::new(data)
    }
}
