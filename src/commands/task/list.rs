use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;

pub(crate) fn list_tasks(mut conn: SqliteConnection, staged: bool) -> Result<(), RemarkError> {
    let tasks = match staged {
        true => database::get_staged_tasks(&mut conn)?,
        false => database::get_all_tasks(&mut conn)?,
    };

    for task in tasks {
        println!("{} {} {}", task.id, task.date, task.name);
    }

    Ok(())
}
