use crate::database;
use diesel::SqliteConnection;
use std::fs;

use crate::errors::RemarkError;
use crate::utils::{get_path, RemarkDir};

pub(crate) fn remove_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let task = database::get_task_like(&mut conn, id.as_str())?;

    database::remove_task(&mut conn, task.id.as_str())?;

    let task_path = get_path(RemarkDir::Task)?.join(format!("{}.md", task.id));

    fs::remove_file(task_path)?;

    println!("removed task '{}'", task.id);

    Ok(())
}
