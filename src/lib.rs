pub mod blob;
pub mod blobstore;
pub mod bloco;
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;
