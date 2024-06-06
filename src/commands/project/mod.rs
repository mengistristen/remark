use diesel::SqliteConnection;

use crate::cli::ProjectAction;
use crate::errors::RemarkError;

use self::add::add_project;
use self::edit::edit_project;
use self::list::list_projects;
use self::remove::remove_project;

mod add;
mod edit;
mod list;
mod remove;

pub fn process_project(conn: SqliteConnection, action: ProjectAction) -> Result<(), RemarkError> {
    match action {
        ProjectAction::Add { name } => add_project(conn, name)?,
        ProjectAction::List => list_projects(conn)?,
        ProjectAction::Edit { id } => edit_project(conn, id)?,
        ProjectAction::Remove { id } => remove_project(conn, id)?,
    };

    Ok(())
}
