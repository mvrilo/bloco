use crate::Hash;
use bincode::{config::Configuration, Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Ref {
    pub name: String,
    pub bucket: String,
    pub size: u64,
    pub chunks: Vec<Hash>,
    // permissions
}

impl Ref {
    pub fn new(name: String, bucket: String, size: u64, chunks: Vec<Hash>) -> Self {
        Ref {
            name,
            bucket,
            chunks,
            size,
        }
    }
}
