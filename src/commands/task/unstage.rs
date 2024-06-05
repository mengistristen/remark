use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;

pub(crate) fn unstage_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let pattern = format!("{}%", id);
    let mut tasks = database::get_tasks_like(&mut conn, pattern)?;

    if tasks.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one item with ID beginning with {}",
            id
        )));
    }

    let task = tasks.remove(0);

    database::mark_task(&mut conn, id, false)?;

    println!("unstaged task '{}'", task.id);

    Ok(())
}
