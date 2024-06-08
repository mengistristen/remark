use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;
use crate::utils::get_date_or_default;

pub(crate) fn list_tasks(
    mut conn: SqliteConnection,
    from: Option<chrono::NaiveDate>,
    to: Option<chrono::NaiveDate>,
) -> Result<(), RemarkError> {
    let from = get_date_or_default(from, chrono::NaiveDate::MIN);
    let to = get_date_or_default(to, chrono::Local::now().naive_local().into());
    let results = database::get_tasks_in_range(&mut conn, from, to)?;

    for (task, project) in results {
        println!(
            "{} {:30} {} {}",
            task.id, project.name, task.date, task.name
        );
    }

    Ok(())
}
