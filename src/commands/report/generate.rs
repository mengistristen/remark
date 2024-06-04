use crate::commands::report::output_report;
use crate::models::{Report, Task};
use crate::schema::reports;
use crate::schema::tasks::dsl::*;
use crate::utils::{get_path, DataDir};
use diesel::prelude::*;
use std::fs::{self};
use uuid::Uuid;

use crate::errors::RemarkError;

pub(crate) fn generate_report(
    mut conn: SqliteConnection,
    report_name: String,
    skip_marking: bool,
) -> Result<(), RemarkError> {
    let report_id = Uuid::new_v4();
    let path = get_path(DataDir::Report)?.join(format!("{}.md", report_id));
    let result = tasks
        .select(Task::as_select())
        .filter(staged.eq(true))
        .order(date.desc())
        .load(&mut conn)?;

    if result.is_empty() {
        return Err(RemarkError::Error(
            "cannot create a report with no tasks".to_owned(),
        ));
    }

    // create the report file in a new scope so that it is closed
    // before the program might try to remove it
    {
        let report_file = fs::File::create_new(path.clone())?;

        output_report(report_file, &result, &report_name)?;
    }

    if !skip_marking {
        for task in result {
            diesel::update(tasks.find(task.id))
                .set(staged.eq(false))
                .execute(&mut conn)?;
        }
    }

    let report = Report {
        id: report_id.to_string(),
        name: report_name,
    };

    if let Err(err) = diesel::insert_into(reports::table)
        .values(report)
        .execute(&mut conn)
    {
        fs::remove_file(path)?;
        return Err(err.into());
    }

    println!("generated report '{}'", report_id);

    Ok(())
}
