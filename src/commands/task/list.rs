use crate::models::Task;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;

use crate::errors::RemarkError;

pub(crate) fn list_tasks(mut conn: SqliteConnection, task_staged: bool) -> Result<(), RemarkError> {
    let result = match task_staged {
        true => tasks
            .select(Task::as_select())
            .filter(staged.eq(true))
            .order(date.desc())
            .load(&mut conn)?,
        false => tasks
            .select(Task::as_select())
            .order(date.desc())
            .load(&mut conn)?,
    };

    for task in result {
        println!("{} {} {}", task.id, task.date, task.name);
    }

    Ok(())
}
