use diesel::SqliteConnection;
use std::io::{self, Write};

use crate::{
    data::MdFile,
    database,
    errors::RemarkError,
    serializable::SerializableTask,
    utils::{get_path, RemarkDir},
};

pub(crate) fn print_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let task = database::get_task_like(&mut conn, id.as_str())?;
    let task_path = get_path(RemarkDir::Task)?.join(format!("{}.md", task.id));
    let file = MdFile::<SerializableTask>::from_file(&task_path)?;

    io::stdout().write_all(file.content.as_bytes())?;

    Ok(())
}
