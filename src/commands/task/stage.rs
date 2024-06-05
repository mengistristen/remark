use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;

pub(crate) fn stage_task(
    mut conn: SqliteConnection,
    id: String,
    staged: bool,
) -> Result<(), RemarkError> {
    let pattern = format!("{}%", id);
    let task = database::get_task_like(&mut conn, &pattern)?;

    database::mark_task(&mut conn, task.id.clone(), staged)?;

    println!(
        "{} task '{}'",
        if staged { "staged" } else { "unstaged" },
        task.id
    );

    Ok(())
}
