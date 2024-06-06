use diesel::SqliteConnection;

use crate::{cli::TaskAction, errors::RemarkError};

use self::{
    add::add_task, edit::edit_task, list::list_tasks, remove::remove_task, stage::stage_task,
};

mod add;
mod edit;
mod list;
mod remove;
mod stage;

pub fn process_task(conn: SqliteConnection, action: TaskAction) -> Result<(), RemarkError> {
    match action {
        TaskAction::Add { project } => add_task(conn, project)?,
        TaskAction::List { staged } => list_tasks(conn, staged)?,
        TaskAction::Edit { id, metadata } => edit_task(conn, id, metadata)?,
        TaskAction::Stage { id } => stage_task(conn, id, true)?,
        TaskAction::UnStage { id } => stage_task(conn, id, false)?,
        TaskAction::Remove { id } => remove_task(conn, id)?,
    };

    Ok(())
}
