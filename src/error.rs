// use bincode::{Decode, Encode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("bincode decode error")]
    DecodeError(#[from] bincode::error::DecodeError),

    #[error("database error")]
    SledError(#[from] sled::Error),

    #[error("not found")]
    NotFound,
}

// #[derive(Debug, Clone)]
// pub struct DecodeError;

// impl From<bincode::error::DecodeError> for DecodeError {
//     fn from(err: bincode::error::DecodeError) -> Self {
//         Self {}
//     }
// }
