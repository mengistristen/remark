use crate::commands::report::output_report;
use crate::database;
use crate::models::Report;
use crate::utils::{get_path, DataDir};
use diesel::SqliteConnection;
use std::fs;
use uuid::Uuid;

use crate::errors::RemarkError;

pub(crate) fn generate_report(
    mut conn: SqliteConnection,
    report_name: String,
    skip_marking: bool,
) -> Result<(), RemarkError> {
    let report_id = Uuid::new_v4();
    let path = get_path(DataDir::Report)?.join(format!("{}.md", report_id));
    let tasks = database::get_staged_tasks(&mut conn)?;

    if tasks.is_empty() {
        return Err(RemarkError::Error(
            "cannot create a report with no tasks".to_owned(),
        ));
    }

    // create the report file in a new scope so that it is closed
    // before the program might try to remove it
    {
        let report_file = fs::File::create_new(path.clone())?;

        output_report(report_file, &tasks, &report_name)?;
    }

    if !skip_marking {
        for task in tasks {
            database::mark_task(&mut conn, task.id, true)?;
        }
    }

    let report = Report {
        id: report_id.to_string(),
        name: report_name,
    };

    if let Err(err) = database::insert_report(&mut conn, &report) {
        fs::remove_file(path)?;
        return Err(err);
    }

    println!("generated report '{}'", report_id);

    Ok(())
}
