use crate::data::MdFile;
use crate::errors::RemarkError;
use crate::models::Project;
use crate::schema::projects;
use crate::utils::{get_path, launch_editor, DataDir};
use diesel::prelude::*;
use std::fs;
use tempfile::NamedTempFile;
use uuid::Uuid;

pub(crate) fn add_project(mut conn: SqliteConnection, name: String) -> Result<(), RemarkError> {
    let id = Uuid::new_v4();
    let file = NamedTempFile::new()?;

    let contents = launch_editor(file)?;
    let project = Project {
        id: id.to_string(),
        name,
    };

    // save to file
    let md_file = MdFile::new(project.clone(), contents);

    let final_path = get_path(DataDir::Project)?.join(format!("{}.md", id));

    md_file.save(&final_path)?;

    // save to DB
    if let Err(err) = diesel::insert_into(projects::table)
        .values(&project)
        .execute(&mut conn)
    {
        fs::remove_file(final_path)?;
        return Err(err.into());
    }

    println!("created project '{}'", id);

    Ok(())
}
