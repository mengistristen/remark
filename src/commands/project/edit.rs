use std::io::Write;

use crate::{
    data::MdFile,
    database,
    errors::RemarkError,
    models::Project,
    utils::{get_path, launch_editor, DataDir},
};
use diesel::SqliteConnection;
use tempfile::NamedTempFile;

pub(crate) fn edit_project(
    mut conn: SqliteConnection,
    project_id: String,
) -> Result<(), RemarkError> {
    let pattern = format!("{}%", project_id);
    let mut projects = database::get_projects_like(&mut conn, pattern)?;

    if projects.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one item with ID beginning with {}",
            project_id
        )));
    }

    let project = projects.remove(0);
    let filename = format!("{}.md", project.id);
    let path = get_path(DataDir::Project)?.join(filename);

    let project_file = MdFile::<Project>::from_file(&path)?;

    let mut file = NamedTempFile::new()?;

    file.write_all(project_file.content.as_bytes())?;

    let contents = launch_editor(file)?;

    let new_file = MdFile::new(project, contents);

    new_file.save(&path)?;

    println!("successfully edited project");

    Ok(())
}
