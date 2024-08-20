use crate::data::MdFile;
use crate::database;
use crate::errors::RemarkError;
use crate::models::Project;
use crate::serializable::SerializableProject;
use crate::utils::{get_path, launch_editor, prompt_user, RemarkDir};
use diesel::SqliteConnection;
use std::fs;
use tempfile::NamedTempFile;
use uuid::Uuid;

pub(crate) fn add_project(mut conn: SqliteConnection) -> Result<(), RemarkError> {
    let id = Uuid::new_v4();
    let file = NamedTempFile::new()?;

    let project = Project {
        id: id.to_string(),
        name: prompt_user("Name")?,
    };
    let contents = launch_editor(file)?;

    // save to file
    let md_file = MdFile::<SerializableProject>::new((&project).into(), contents);

    let final_path = get_path(RemarkDir::Project)?.join(format!("{}.md", id));

    md_file.save(&final_path)?;

    // save to DB
    if let Err(err) = database::insert_project(&mut conn, &project) {
        fs::remove_file(final_path)?;
        return Err(err);
    }

    println!("created project '{}'", id);

    Ok(())
}
