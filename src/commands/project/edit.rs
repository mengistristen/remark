use std::io::Write;

use crate::schema::projects::dsl::*;
use crate::{
    data::MdFile,
    errors::RemarkError,
    models::Project,
    utils::{get_path, launch_editor, DataDir},
};
use diesel::prelude::*;
use tempfile::NamedTempFile;

pub(crate) fn edit_project(
    mut conn: SqliteConnection,
    project_id: String,
) -> Result<(), RemarkError> {
    let pattern = format!("{}%", project_id);
    let mut result = projects
        .filter(id.like(pattern))
        .select(Project::as_select())
        .load(&mut conn)?;

    if result.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one item with ID beginning with {}",
            project_id
        )));
    }

    let project = result.remove(0);
    let filename = format!("{}.md", project.id);
    let path = get_path(DataDir::Project)?.join(filename);

    let project_file = MdFile::<Project>::from_file(path.clone())?;

    let mut file = NamedTempFile::new()?;

    file.write(project_file.content.as_bytes())?;

    let contents = launch_editor(file)?;

    let new_file = MdFile::new(project, contents);

    new_file.save(path)?;

    println!("successfully edited project");

    Ok(())
}
