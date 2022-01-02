use crate::Hash;
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct FileRef {
    pub name: String,
    pub hash: String,
    pub size: i64,
}

impl FileRef {
    pub fn new(name: String, size: i64, hash: Hash) -> Self {
        let hash = hash.as_hex();
        FileRef { name, hash, size }
    }
}
