use crate::data::MdFile;
use crate::utils::{get_project_path, DataDir};
use crate::ProjectError;
use std::env;
use std::fs;
use std::process;
use tempfile::NamedTempFile;
use uuid::Uuid;

use super::Project;

pub(crate) fn create_project(name: String) -> Result<(), ProjectError> {
    let id = Uuid::new_v4();
    let file = NamedTempFile::new()?;

    let editor = env::var("EDITOR").unwrap_or("vim".to_owned());
    let status = process::Command::new(editor)
        .arg(file.path().as_os_str())
        .status()?;

    if !status.success() {
        return Err(ProjectError::EditorError);
    }

    let contents = fs::read_to_string(file.path())?;
    let metadata = Project::new(id.to_string(), name);
    let md_file = MdFile::new(metadata, contents);

    let final_path = get_project_path(DataDir::Project)?.join(format!("{}.md", id));

    md_file.save(final_path)?;

    Ok(())
}
