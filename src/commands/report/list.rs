use crate::schema::reports::dsl::*;
use diesel::prelude::*;

use crate::{errors::RemarkError, models::Report};

pub(crate) fn list_reports(mut conn: SqliteConnection) -> Result<(), RemarkError> {
    let result = reports.select(Report::as_select()).load(&mut conn)?;

    for report in result {
        println!("{} {}", report.id, report.name);
    }

    Ok(())
}
