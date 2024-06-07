use diesel::SqliteConnection;

use crate::{cli::TaskAction, errors::RemarkError};

use self::{
    add::add_task, edit::edit_task, list::list_tasks, print::print_task, remove::remove_task,
};

mod add;
mod edit;
mod list;
mod print;
mod remove;

pub fn process_task(conn: SqliteConnection, action: TaskAction) -> Result<(), RemarkError> {
    match action {
        TaskAction::Add { project } => add_task(conn, project)?,
        TaskAction::List { from, to } => list_tasks(conn, from, to)?,
        TaskAction::Edit { id, metadata } => edit_task(conn, id, metadata)?,
        TaskAction::Remove { id } => remove_task(conn, id)?,
        TaskAction::Print { id } => print_task(conn, id)?,
    };

    Ok(())
}
