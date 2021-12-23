#[derive(Debug, Clone, Default, PartialEq)]
pub struct Blob {
    pub data: Option<Vec<u8>>,
}

impl Blob {
    pub fn hash(&self) -> [u8; 32] {
        let data: Vec<u8> = match &self.data {
            Some(data) => data.to_vec(),
            None => vec![],
        };
        blake3::hash(&data).into()
    }
}

impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {
        let data = Some(data);
        Blob { data }
    }
}
