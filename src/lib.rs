use std::io;
use thiserror::Error;

pub mod cli;
pub mod commands;
pub mod data;
pub mod utils;

#[derive(Error, Debug)]
pub enum ProjectError {
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
