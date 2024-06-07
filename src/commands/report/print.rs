use std::io;

use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;
use crate::utils::get_default_date;

use super::output_report;

pub(crate) fn print_report(
    mut conn: SqliteConnection,
    from: chrono::NaiveDate,
    to: Option<chrono::NaiveDate>,
) -> Result<(), RemarkError> {
    let to = get_default_date(to);

    let items = database::get_tasks_in_range(&mut conn, from, to)?;
    let name = "Current".to_owned();

    output_report(io::stdout(), &items, &name)?;

    Ok(())
}
