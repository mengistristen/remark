use diesel::SqliteConnection;
use std::fs;

use crate::{
    database,
    errors::RemarkError,
    utils::{get_path, RemarkDir},
};

pub(crate) fn remove_report(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let report = database::get_report_like(&mut conn, &id)?;

    database::remove_report(&mut conn, &report.id)?;

    let report_path = get_path(RemarkDir::Report)?.join(format!("{}.md", report.id));

    fs::remove_file(report_path)?;

    println!("removed report '{}'", report.id);

    Ok(())
}
