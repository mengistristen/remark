use diesel::SqliteConnection;
use std::fs;

use crate::{
    database,
    errors::RemarkError,
    utils::{get_path, RemarkDir},
};

pub(crate) fn remove_project(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let project = database::get_project_like(&mut conn, &id)?;

    database::remove_project(&mut conn, &project.id)?;

    let project_path = get_path(RemarkDir::Project)?.join(format!("{}.md", project.id));

    fs::remove_file(project_path)?;

    println!("removed project '{}'", project.id);

    Ok(())
}
