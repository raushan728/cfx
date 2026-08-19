use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CfxError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("Invalid password or corrupted CFX file")]
    IncorrectPassword,

    #[error("File too large. Maximum supported size is 10 GiB.")]
    FileTooLarge,

    #[error("Output file already exists. Use --force to overwrite: {0}")]
    AlreadyExists(PathBuf),

    #[error("Invalid CFX file format: {0}")]
    InvalidFormat(String),

    #[error("Not a regular file: {0}")]
    NotARegularFile(PathBuf),

    #[error("Password prompt error: {0}")]
    PasswordPrompt(String),
}

pub type Result<T> = std::result::Result<T, CfxError>;
