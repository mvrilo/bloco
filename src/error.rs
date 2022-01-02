use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("sqlx database error")]
    SqlxError(#[from] sqlx::error::Error),

    #[error("orion crypto error")]
    UnknownCryptoError(#[from] orion::errors::UnknownCryptoError),

    #[error("utf8 error")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("not found")]
    NotFound,
}
