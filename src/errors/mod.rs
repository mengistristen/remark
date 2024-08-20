use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemarkError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("file persistance error: {0}")]
    PersistError(#[from] tempfile::PersistError),
    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("strip prefix error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error("editor error")]
    EditorError,
    #[error("invalid file error")]
    InvalidFile,
    #[error("error: {0}")]
    Error(String),
}
