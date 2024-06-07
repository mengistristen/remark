use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;
use crate::utils::get_default_date;

pub(crate) fn list_tasks(
    mut conn: SqliteConnection,
    from: chrono::NaiveDate,
    to: Option<chrono::NaiveDate>,
) -> Result<(), RemarkError> {
    let to = get_default_date(to);
    let results = database::get_tasks_in_range(&mut conn, from, to)?;

    for (task, project) in results {
        println!(
            "{} {:30} {} {}",
            task.id, project.name, task.date, task.name
        );
    }

    Ok(())
}
