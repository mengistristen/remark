use std::io::Write;
use std::{env, fs, io, process, str::FromStr};

use tempfile::NamedTempFile;

use crate::errors::RemarkError;

pub enum DataDir {
    Project,
    Task,
    Report,
}

pub fn get_base_dir() -> std::path::PathBuf {
    let mut data_dir = dirs::data_local_dir().expect("failed to find data directory");

    data_dir.push("remark");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");

    data_dir
}

pub fn get_path(dir: DataDir) -> Result<std::path::PathBuf, std::io::Error> {
    let base_path = get_base_dir();
    let path = match dir {
        DataDir::Project => base_path.join("projects"),
        DataDir::Task => base_path.join("tasks"),
        DataDir::Report => base_path.join("reports"),
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

pub(crate) fn prompt_user<T: FromStr>(prompt: &str) -> Result<T, RemarkError> {
    print!("{prompt}: ");
    io::stdout().flush()?;

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let converted = T::from_str(input.trim())
        .map_err(|_| RemarkError::Error("error converting from string".to_owned()))?;

    Ok(converted)
}
