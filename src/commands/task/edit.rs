use std::io::Write;

use crate::data::MdFile;
use crate::database;
use crate::models::Task;
use diesel::SqliteConnection;
use tempfile::NamedTempFile;

use crate::errors::RemarkError;
use crate::utils::{get_path, launch_editor, DataDir};

pub(crate) fn edit_task(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let task = database::get_task_like(&mut conn, &id)?;

    let filename = format!("{}.md", task.id);
    let path = get_path(DataDir::Task)?.join(filename);

    let task_file = MdFile::<Task>::from_file(&path)?;

    let mut file = NamedTempFile::new()?;

    file.write_all(task_file.content.as_bytes())?;

    let contents = launch_editor(file)?;

    let new_file = MdFile::new(task, contents);

    new_file.save(&path)?;

    println!("successfully edited task");

    Ok(())
}
