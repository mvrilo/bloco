use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_hex(self) -> String {
        hex::encode(self.to_vec())
    }
}

impl From<[u8; 32]> for Hash {
    fn from(arr: [u8; 32]) -> Hash {
        Hash(arr)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Blob {
    pub hash: Hash,
    pub data: Vec<u8>,
}

impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {
        let orig_hash: [u8; 32] = blake3::hash(&data).into();
        let hash: Hash = orig_hash.into();
        Blob { hash, data }
    }
}

impl From<File> for Blob {
    fn from(mut file: File) -> Blob {
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let orig_hash: [u8; 32] = blake3::hash(&data).into();
        let hash: Hash = orig_hash.into();
        Blob { hash, data }
    }
}
