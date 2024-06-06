use std::fs::File;
use std::io::{self, Read, Write};

use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;
use crate::utils::{get_path, DataDir};

use super::output_report;

pub(crate) fn print_report(
    mut conn: SqliteConnection,
    id: Option<String>,
) -> Result<(), RemarkError> {
    if let Some(id) = id {
        let report = database::get_report_like(&mut conn, &id)?;
        let report_path = get_path(DataDir::Report)?.join(format!("{}.md", report.id));
        let mut file = File::open(report_path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        io::stdout().write_all(contents.as_bytes())?;
    } else {
        let items = database::get_staged_tasks(&mut conn)?;
        let name = "Current".to_owned();

        output_report(io::stdout(), &items, &name)?;
    }

    Ok(())
}
