use std::io;

use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;
use crate::utils::get_date_or_default;

use super::output_report;

pub(crate) fn print_report(
    mut conn: SqliteConnection,
    from: Option<chrono::NaiveDate>,
    to: Option<chrono::NaiveDate>,
) -> Result<(), RemarkError> {
    let from = get_date_or_default(from, chrono::NaiveDate::MIN);
    let to = get_date_or_default(to, chrono::Local::now().naive_local().into());

    let items = database::get_tasks_in_range(&mut conn, from, to)?;
    let name = "Current".to_owned();

    output_report(io::stdout(), &items, &name)?;

    Ok(())
}
