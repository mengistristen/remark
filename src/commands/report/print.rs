use std::fs::File;
use std::io::{self, Read, Write};

use crate::models::{Report, Task};
use crate::schema::reports::dsl::{id as report_id, reports};
use crate::schema::tasks::dsl::{staged, tasks};
use diesel::prelude::*;

use crate::errors::RemarkError;
use crate::utils::{get_path, DataDir};

use super::output_report;

pub(crate) fn print_report(
    mut conn: SqliteConnection,
    id: Option<String>,
) -> Result<(), RemarkError> {
    if let Some(id) = id {
        let pattern = format!("{}%", id);
        let mut result = reports
            .filter(report_id.like(pattern))
            .select(Report::as_select())
            .load(&mut conn)?;

        if result.len() != 1 {
            return Err(RemarkError::Error(format!(
                "found more than one item with ID beginning with {}",
                id
            )));
        }

        let report = result.remove(0);
        let report_path = get_path(DataDir::Report)?.join(format!("{}.md", report.id));
        let mut file = File::open(report_path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        io::stdout().write_all(contents.as_bytes())?;
    } else {
        let items = tasks
            .filter(staged.eq(true))
            .select(Task::as_select())
            .load(&mut conn)?;
        let name = "Current".to_owned();

        output_report(io::stdout(), &items, &name)?;
    }

    Ok(())
}
