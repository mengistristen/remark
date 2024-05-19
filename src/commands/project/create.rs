use tempfile::NamedTempFile;
use uuid::Uuid;
use crate::ProjectError;
use std::fs;
use std::process;
use std::env;
use std::io::Write;
use crate::utils::{get_project_path, DataDir};

pub(crate) fn create_list(name: String) -> Result<(), ProjectError> {
    let id = Uuid::new_v4();
    let mut file = NamedTempFile::new()?;

    writeln!(file, "{} {}", id, name)?;

    let editor = env::var("EDITOR").unwrap_or("vim".to_owned());

    let status = process::Command::new(editor).arg(file.path().as_os_str()).status()?; 

    if !status.success() {
        return Err(ProjectError::EditorError)
    }

    let final_path = get_project_path(DataDir::Project)?.join(format!("{}.md", id));
                                         
    fs::copy(file, final_path)?;

    Ok(())
}
