use diesel::SqliteConnection;
use std::fs;

use crate::{
    database,
    errors::RemarkError,
    utils::{get_path, DataDir},
};

pub(crate) fn remove_project(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let project = database::get_project_like(&mut conn, &id)?;

    database::remove_tasks_for_project(&mut conn, &project.id)?;
    database::remove_project(&mut conn, &project.id)?;

    let project_path = get_path(DataDir::Project)?.join(format!("{}.md", project.id));

    fs::remove_file(project_path)?;

    println!("removed project '{}'", project.id);

    Ok(())
}
