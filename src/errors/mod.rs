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
    #[error("editor error")]
    EditorError,
    #[error("invalid file error")]
    InvalidFile,
}
