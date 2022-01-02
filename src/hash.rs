#[derive(Clone, Copy, PartialEq)]
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

impl From<blake3::Hash> for Hash {
    fn from(arr: blake3::Hash) -> Hash {
        let arr: [u8; 32] = arr.into();
        Hash(arr)
    }
}

impl From<Vec<u8>> for Hash {
    fn from(arr: Vec<u8>) -> Hash {
        Hash(arr.try_into().unwrap())
    }
}
