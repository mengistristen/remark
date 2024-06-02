use diesel::SqliteConnection;

use crate::{cli::ReportAction, errors::RemarkError};

use self::generate::generate_report;

mod generate;

pub fn process_report(conn: SqliteConnection, action: ReportAction) -> Result<(), RemarkError> {
    match action {
        ReportAction::Generate { skip_marking } => generate_report(conn, skip_marking)?,
    };

    Ok(())
}
