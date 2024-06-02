use crate::models::Task;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;

use crate::errors::RemarkError;

pub(crate) fn generate_report(
    mut conn: SqliteConnection,
    _skip_marking: bool,
) -> Result<(), RemarkError> {
    tasks
        .select(Task::as_select())
        .filter(staged.eq(true))
        .order(date.asc())
        .load(&mut conn)?;
    todo!()
}
