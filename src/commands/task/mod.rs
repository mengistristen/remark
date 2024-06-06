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
        TaskAction::Add {
            project,
            name,
            hours,
            date,
        } => add_task(conn, project, name, hours, date)?,
        TaskAction::List { staged } => list_tasks(conn, staged)?,
        TaskAction::Edit { id } => edit_task(conn, id)?,
        TaskAction::Stage { id } => stage_task(conn, id, true)?,
        TaskAction::UnStage { id } => stage_task(conn, id, false)?,
        TaskAction::Remove { id } => remove_task(conn, id)?,
    };

    Ok(())
}
