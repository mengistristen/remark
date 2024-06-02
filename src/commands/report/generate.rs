use diesel::prelude::*;
use crate::schema::tasks::dsl::*;
use crate::models::Task;

use crate::errors::RemarkError;

pub(crate) fn generate_report(mut conn: SqliteConnection, skip_marking: bool) -> Result<(), RemarkError> {
    tasks.select(Task::as_select()).filter(staged.eq(true)).order(date.asc()).load(&mut conn)?;
    todo!() 
}
