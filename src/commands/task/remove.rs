use crate::database;
use diesel::SqliteConnection;
use std::fs;

use crate::errors::RemarkError;
use crate::utils::{get_path, DataDir};

pub(crate) fn remove_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let task = database::get_task_like(&mut conn, &id)?;

    database::remove_task(&mut conn, &task.id)?;

    let task_path = get_path(DataDir::Task)?.join(format!("{}.md", task.id));

    fs::remove_file(task_path)?;

    println!("removed task '{}'", task.id);

    Ok(())
}
