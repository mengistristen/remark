use diesel::prelude::*;
use std::fs;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::data::MdFile;
use crate::errors::RemarkError;
use crate::models::{Project, Task};
use crate::schema::projects::dsl::*;
use crate::schema::tasks;
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
    let mut result = projects
        .filter(id.like(pattern))
        .select(Project::as_select())
        .load(&mut conn)?;

    if result.len() != 1 {
        return Err(RemarkError::Error(
            "found more than one item with ID beginning with {}".to_owned(),
        ));
    }

    let project = result.remove(0);
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
    if let Err(err) = diesel::insert_into(tasks::table)
        .values(&task)
        .execute(&mut conn)
    {
        fs::remove_file(final_path)?;
        return Err(err.into());
    }

    println!("created task '{}'", task_id);

    Ok(())
}
