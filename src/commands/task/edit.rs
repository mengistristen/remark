use std::io::Write;

use crate::data::MdFile;
use crate::database;
use crate::serializable::{SerializableTask, UpdateTask};
use diesel::SqliteConnection;
use tempfile::NamedTempFile;

use crate::errors::RemarkError;
use crate::utils::{get_path, launch_editor, RemarkDir};

pub(crate) fn edit_task(
    mut conn: SqliteConnection,
    id: String,
    metadata: bool,
) -> Result<(), RemarkError> {
    let task = database::get_task_like(&mut conn, &id)?;

    let filename = format!("{}.md", task.id);
    let path = get_path(RemarkDir::Task)?.join(filename);

    let task_file = MdFile::<SerializableTask>::from_file(&path)?;

    let mut file = NamedTempFile::new()?;

    if metadata {
        let update = UpdateTask::from_task(&mut conn, &task)?;
        let metadata_str = serde_yaml::to_string(&update)?;

        file.write_all(metadata_str.as_bytes())?;
    } else {
        file.write_all(task_file.content.as_bytes())?;
    }

    let contents = launch_editor(file)?;

    if metadata {
        let deserialized = serde_yaml::from_str(contents.as_str())?;
        let new_task = database::update_task(&mut conn, &task.id, &deserialized)?;

        let new_file = MdFile::new(
            SerializableTask::from_task(&mut conn, &new_task)?,
            task_file.content,
        );

        new_file.save(&path)?;
    } else {
        let new_file = MdFile::new(SerializableTask::from_task(&mut conn, &task)?, contents);

        new_file.save(&path)?;
    }

    println!("successfully edited task");

    Ok(())
}
