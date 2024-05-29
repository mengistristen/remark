use std::{env, process, fs};

use tempfile::NamedTempFile;

use crate::errors::RemarkError;

pub enum DataDir {
    Project,
    Task,
}

fn get_base_dir() -> std::path::PathBuf {
    let mut data_dir = dirs::data_local_dir().expect("failed to find data directory");

    data_dir.push("renew");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");

    data_dir
}

pub fn get_project_path(dir: DataDir) -> Result<std::path::PathBuf, std::io::Error> {
    let base_path = get_base_dir();
    let path = match dir {
        DataDir::Project => base_path.join("projects"),
        DataDir::Task => base_path.join("tasks"),
    };

    if !path.exists() {
        fs::create_dir_all(path.clone())?;
    }

    Ok(path)
}

pub(crate) fn launch_editor(file: NamedTempFile) -> Result<String, RemarkError> {
    let editor = env::var("EDITOR").unwrap_or("vim".to_owned());
    let status = process::Command::new(editor)
        .arg(file.path().as_os_str())
        .status()?;

    if !status.success() {
        return Err(RemarkError::EditorError);
    }

    let contents = fs::read_to_string(file.path())?;
   
    Ok(contents)
}
