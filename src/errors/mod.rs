use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemarkError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("file persistance error")]
    PersistError(#[from] tempfile::PersistError),
    #[error("yaml error")]
    YamlError(#[from] serde_yaml::Error),
    #[error("database error")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("editor error")]
    EditorError,
    #[error("invalid file error")]
    InvalidFile,
    #[error("remark error")]
    Error(String),
}
