use std::io::Write;

use crate::{
    data::MdFile,
    database,
    errors::RemarkError,
    serializable::{SerializableProject, UpdateProject},
    utils::{get_path, launch_editor, RemarkDir},
};
use diesel::SqliteConnection;
use tempfile::NamedTempFile;

pub(crate) fn edit_project(
    mut conn: SqliteConnection,
    id: String,
    metadata: bool,
) -> Result<(), RemarkError> {
    let project = database::get_project_like(&mut conn, &id)?;

    let filename = format!("{}.md", project.id);
    let path = get_path(RemarkDir::Project)?.join(filename);

    let project_file = MdFile::<SerializableProject>::from_file(&path)?;

    let mut file = NamedTempFile::new()?;

    if metadata {
        let update = UpdateProject::from_project(&project);
        let metadata_str = serde_yaml::to_string(&update)?;

        file.write_all(metadata_str.as_bytes())?;
    } else {
        file.write_all(project_file.content.as_bytes())?;
    }

    let contents = launch_editor(file)?;

    if metadata {
        let deserialized = serde_yaml::from_str(contents.as_str())?;
        let new_project = database::update_project(&mut conn, project.id, &deserialized)?;

        let new_file =
            MdFile::<SerializableProject>::new((&new_project).into(), project_file.content);

        new_file.save(&path)?;
    } else {
        let new_file = MdFile::<SerializableProject>::new((&project).into(), contents);

        new_file.save(&path)?;
    }

    println!("successfully edited project");

    Ok(())
}
