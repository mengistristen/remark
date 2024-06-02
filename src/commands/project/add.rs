use crate::data::MdFile;
use crate::errors::RemarkError;
use crate::utils::{get_project_path, launch_editor, DataDir};
use tempfile::NamedTempFile;
use uuid::Uuid;

use super::Project;

pub(crate) fn add_project(name: String) -> Result<(), RemarkError> {
    let id = Uuid::new_v4();
    let file = NamedTempFile::new()?;

    let contents = launch_editor(file)?;
    let metadata = Project::new(id.to_string(), name);
    let md_file = MdFile::new(metadata, contents);

    let final_path = get_project_path(DataDir::Project)?.join(format!("{}.md", id));

    md_file.save(final_path)?;

    Ok(())
}
