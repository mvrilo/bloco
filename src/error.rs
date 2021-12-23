use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("not found")]
    NotFound,
}
