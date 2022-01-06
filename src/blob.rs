use crate::{Hash, Result};
use bytes::{buf::BufMut, BytesMut};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone, PartialEq)]
pub struct Blob(pub BytesMut);

impl Blob {
    pub fn new(data: BytesMut) -> Self {
        Blob(data)
    }

    pub fn zero() -> Self {
        Blob(BytesMut::new())
    }

    pub fn hash(&self) -> Hash {
        blake3::hash(&self.0).into()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub async fn read_file(mut file: fs::File) -> Result<Blob> {
        let mut buf = BytesMut::new();
        file.read_buf(&mut buf).await?;
        Ok(Blob::new(buf))
    }
}

impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {
        let buf = &mut BytesMut::new();
        buf.put(&*data);
        Blob::new(buf.clone())
    }
}

impl From<&[u8]> for Blob {
    fn from(data: &[u8]) -> Blob {
        let buf = &mut BytesMut::new();
        buf.put(&*data);
        Blob::new(buf.clone())
    }
}

impl From<BytesMut> for Blob {
    fn from(data: BytesMut) -> Blob {
        Blob::new(data.clone())
    }
}
