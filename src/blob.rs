use crate::{Hash, Result};
use bincode::{config::Configuration, Decode, Encode};
use lazy_static::lazy_static;
use std::fs;
use std::io::Read;

lazy_static! {
    static ref CONFIG: Configuration = Configuration::standard();
}

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Ref {
    pub name: String,
    pub size: u64,
    pub chunks: Vec<Hash>,
}

impl Ref {
    pub fn new(name: String, size: u64, chunks: Vec<Hash>) -> Self {
        Ref { name, chunks, size }
    }

    pub fn from_vec(arr: &[u8]) -> Result<Vec<Ref>> {
        let (rref, _): (Vec<Ref>, usize) = bincode::decode_from_slice(arr, *CONFIG)?;
        Ok(rref)
    }

    pub fn from_slice(arr: &[u8]) -> Result<Ref> {
        let (rref, _): (Ref, usize) = bincode::decode_from_slice(arr, *CONFIG)?;
        Ok(rref)
    }
}

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Blob(pub Vec<u8>);

impl Blob {
    pub fn new(data: Vec<u8>) -> Self {
        Blob(data)
    }

    pub fn hash(&self) -> Hash {
        blake3::hash(&self.0).into()
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
