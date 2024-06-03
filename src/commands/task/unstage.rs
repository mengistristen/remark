use crate::models::Task;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;

use crate::errors::RemarkError;

pub(crate) fn unstage_task(mut conn: SqliteConnection, task_id: String) -> Result<(), RemarkError> {
    let pattern = format!("{}%", task_id);
    let mut result = tasks
        .filter(id.like(pattern))
        .select(Task::as_select())
        .load(&mut conn)?;

    if result.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one item with ID beginning with {}",
            task_id
        )));
    }

    let task = result.remove(0);

    diesel::update(tasks.filter(id.eq(task.id.clone())))
        .set(staged.eq(false))
        .execute(&mut conn)?;

    println!("unstaged task '{}'", task.id);

    Ok(())
}
