use crate::{utils::{get_project_path, DataDir}, ProjectError, data::MdFile};

use super::Project;

pub(crate) fn list_projects() -> Result<(), ProjectError> {
    let path = get_project_path(DataDir::Project)?; 

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let project = MdFile::<Project>::from_file(path)?;

            println!("{} {}", project.metadata.id, project.metadata.name);
        }
    }

    Ok(())
}
