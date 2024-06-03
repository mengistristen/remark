use diesel::SqliteConnection;

use crate::{cli::ReportAction, errors::RemarkError};

use self::{generate::generate_report, list::list_reports, print::print_report};

mod generate;
mod list;
mod print;

pub fn process_report(conn: SqliteConnection, action: ReportAction) -> Result<(), RemarkError> {
    match action {
        ReportAction::Generate { name, skip_marking } => generate_report(conn, name, skip_marking)?,
        ReportAction::Print { id } => print_report(conn, id)?,
        ReportAction::List => list_reports(conn)?,
    };

    Ok(())
}
