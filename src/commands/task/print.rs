use diesel::SqliteConnection;
use std::io::{self, Write};

use crate::{
    data::MdFile,
    database,
    errors::RemarkError,
    models::Task,
    utils::{get_path, DataDir},
};

pub(crate) fn print_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let task = database::get_task_like(&mut conn, &id)?;
    let task_path = get_path(DataDir::Task)?.join(format!("{}.md", task.id));
    let file = MdFile::<Task>::from_file(&task_path)?;

    io::stdout().write_all(file.content.as_bytes())?;

    Ok(())
}
