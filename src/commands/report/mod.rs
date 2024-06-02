use diesel::SqliteConnection;

use crate::{cli::ReportAction, errors::RemarkError};

mod generate;

pub fn process_report(conn: SqliteConnection, action: ReportAction) -> Result<(), RemarkError> {
    match action {
        ReportAction::Generate { skip_marking } => todo!(), 
    };

    Ok(())
}
