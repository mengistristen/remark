use diesel::SqliteConnection;
use std::fs;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::data::MdFile;
use crate::database;
use crate::errors::RemarkError;
use crate::models::Task;
use crate::utils::{get_path, launch_editor, DataDir};

pub(crate) fn add_task(
    mut conn: SqliteConnection,
    project: String,
    task_name: String,
    hours: f32,
    date: Option<chrono::NaiveDate>,
) -> Result<(), RemarkError> {
    let task_id = Uuid::new_v4();
    let file = NamedTempFile::new()?;

    let pattern = format!("{}%", project);

    let project = database::get_project_like(&mut conn, &pattern)?;
    let task_date = match date {
        Some(date) => date,
        None => chrono::Local::now().naive_local().into(),
    };

    let contents = launch_editor(file)?;
    let task = Task {
        id: task_id.to_string(),
        name: task_name,
        date: task_date,
        staged: true,
        project_id: project.id,
        hours,
    };

    // save to file
    let md_file = MdFile::new(task.clone(), contents);

    let final_path = get_path(DataDir::Task)?.join(format!("{}.md", task_id));

    md_file.save(&final_path)?;

    // save to DB
    if let Err(err) = database::insert_task(&mut conn, &task) {
        fs::remove_file(final_path)?;
        return Err(err);
    }

    println!("created task '{}'", task_id);

    Ok(())
}
