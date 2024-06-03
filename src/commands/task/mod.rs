use diesel::SqliteConnection;

use crate::{cli::TaskAction, errors::RemarkError};

use self::{
    add::add_task, edit::edit_task, list::list_tasks, stage::stage_task, unstage::unstage_task,
};

mod add;
mod edit;
mod list;
mod stage;
mod unstage;

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
        TaskAction::Stage { id } => stage_task(conn, id)?,
        TaskAction::UnStage { id } => unstage_task(conn, id)?,
    };

    Ok(())
}
