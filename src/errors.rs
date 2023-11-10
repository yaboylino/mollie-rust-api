use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MollieError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("environment error: {0}")]
    EnvError(#[from] std::env::VarError),
    #[error("HTTP error: {0}")]
    HttpError(StatusCode),
}

pub type Result<T> = std::result::Result<T, MollieError>;
