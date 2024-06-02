use diesel::SqliteConnection;

use crate::cli::ProjectAction;
use crate::errors::RemarkError;

use self::add::add_project;
use self::edit::edit_project;
use self::list::list_projects;

pub mod add;
pub mod edit;
pub mod list;

pub fn process_project(conn: SqliteConnection, action: ProjectAction) -> Result<(), RemarkError> {
    match action {
        ProjectAction::Add { name } => add_project(conn, name)?,
        ProjectAction::List => list_projects(conn)?,
        ProjectAction::Edit { id } => edit_project(conn, id)?,
    };

    Ok(())
}
