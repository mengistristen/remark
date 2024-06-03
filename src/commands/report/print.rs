use std::fs::File;
use std::io::{self, Read, Write};

use crate::models::Report;
use crate::schema::reports::dsl::*;
use diesel::prelude::*;

use crate::errors::RemarkError;
use crate::utils::{get_path, DataDir};

pub(crate) fn print_report(
    mut conn: SqliteConnection,
    report_id: String,
) -> Result<(), RemarkError> {
    let pattern = format!("{}%", report_id);
    let mut result = reports
        .filter(id.like(pattern))
        .select(Report::as_select())
        .load(&mut conn)?;

    if result.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one item with ID beginning with {}",
            report_id
        )));
    }

    let report = result.remove(0);
    let report_path = get_path(DataDir::Report)?.join(format!("{}.md", report.id));
    let mut file = File::open(report_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    io::stdout().write_all(contents.as_bytes())?;

    Ok(())
}
