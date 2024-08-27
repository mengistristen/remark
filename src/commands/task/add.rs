use chrono::NaiveDate;
use diesel::SqliteConnection;
use std::fs;
use std::str::FromStr;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::data::MdFile;
use crate::database;
use crate::errors::RemarkError;
use crate::models::Task;
use crate::serializable::SerializableTask;
use crate::utils::{get_path, launch_editor, prompt_user, RemarkDir};

pub(crate) fn add_task(mut conn: SqliteConnection, project_id: String) -> Result<(), RemarkError> {
    let task_id = Uuid::new_v4();
    let file = NamedTempFile::new()?;

    let name = prompt_user("Name")?;
    let hours = prompt_user("Hours")?;
    let date_str = prompt_user::<String>("Date (YYYY-MM-DD, default=today)")?;
    let tags_str = prompt_user::<String>("Tags")?;

    let project = database::get_project_like(&mut conn, project_id.as_str())?;

    let date = if date_str.is_empty() {
        chrono::Local::now().naive_local().into()
    } else {
        NaiveDate::from_str(date_str.as_str())
            .map_err(|_| RemarkError::Error("error converting date from string".to_owned()))?
    };

    let tags: Vec<String> = tags_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let task = Task {
        id: task_id.to_string(),
        project_id: project.id,
        name,
        hours,
        date,
    };

    let contents = launch_editor(file)?;

    // save to file
    let md_file = MdFile::new(
        SerializableTask::from_task_with_tags(
            &task,
            if tags.is_empty() { None } else { Some(&tags) },
        ),
        contents,
    );

    let final_path = get_path(RemarkDir::Task)?.join(format!("{}.md", task_id));

    md_file.save(&final_path)?;

    // save to DB
    if let Err(err) = database::insert_task(
        &mut conn,
        &task,
        if tags.is_empty() { None } else { Some(&tags) },
    ) {
        fs::remove_file(final_path)?;
        return Err(err);
    }

    println!("created task '{}'", task_id);

    Ok(())
}
