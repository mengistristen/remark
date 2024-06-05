use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;

pub(crate) fn unstage_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let pattern = format!("{}%", id);
    let task = database::get_task_like(&mut conn, &pattern)?;

    database::mark_task(&mut conn, id, false)?;

    println!("unstaged task '{}'", task.id);

    Ok(())
}
