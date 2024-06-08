use crate::commands::report::output_report;
use crate::database;
use crate::models::Report;
use crate::utils::{get_date_or_default, get_path, RemarkDir};
use diesel::SqliteConnection;
use std::fs;
use uuid::Uuid;

use crate::errors::RemarkError;

pub(crate) fn generate_report(
    mut conn: SqliteConnection,
    name: Option<String>,
    from: chrono::NaiveDate,
    to: Option<chrono::NaiveDate>,
) -> Result<(), RemarkError> {
    let to = get_date_or_default(to, chrono::Local::now().naive_local().into());
    let name = match name {
        Some(name) => name,
        None => format!("{} to {}", from, to),
    };

    let report_id = Uuid::new_v4();
    let path = get_path(RemarkDir::Report)?.join(format!("{}.md", report_id));
    let tasks = database::get_tasks_in_range(&mut conn, from, to)?;

    if tasks.is_empty() {
        return Err(RemarkError::Error(
            "cannot create a report with no tasks".to_owned(),
        ));
    }

    // create the report file in a new scope so that it is closed
    // before the program might try to remove it
    {
        let report_file = fs::File::create_new(path.clone())?;

        output_report(report_file, &tasks, &name)?;
    }

    let report = Report {
        id: report_id.to_string(),
        name,
    };

    if let Err(err) = database::insert_report(&mut conn, &report) {
        fs::remove_file(path)?;
        return Err(err);
    }

    println!("generated report '{}'", report_id);

    Ok(())
}
