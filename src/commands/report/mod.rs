use diesel::SqliteConnection;

use crate::{cli::ReportAction, errors::RemarkError};

use self::generate::generate_report;

mod generate;

pub fn process_report(conn: SqliteConnection, action: ReportAction) -> Result<(), RemarkError> {
    match action {
        ReportAction::Generate { name, skip_marking } => generate_report(conn, name, skip_marking)?,
    };

    Ok(())
}
