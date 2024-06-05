use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;

pub(crate) fn list_reports(mut conn: SqliteConnection) -> Result<(), RemarkError> {
    let reports = database::get_all_reports(&mut conn)?;

    for report in reports {
        println!("{} {}", report.id, report.name);
    }

    Ok(())
}
