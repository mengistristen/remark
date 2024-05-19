use thiserror::Error;
use std::io;

pub mod utils;
pub mod commands;
pub mod cli;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("file persistance error")]
    PersistError(#[from] tempfile::PersistError),
    #[error("editor error")]
    EditorError,
}

