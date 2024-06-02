use serde::{Deserialize, Serialize};

use crate::cli::ProjectAction;
use crate::errors::RemarkError;

use self::add::add_project;
use self::edit::edit_project;
use self::list::list_projects;

pub mod add;
pub mod list;
pub mod edit;

#[derive(Serialize, Deserialize)]
pub(crate) struct Project {
    id: String,
    name: String,
}

impl Project {
    pub(crate) fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

pub fn process_project(action: ProjectAction) -> Result<(), RemarkError> {
    match action {
        ProjectAction::Add { name } => add_project(name)?,
        ProjectAction::List => list_projects()?, 
        ProjectAction::Edit { id } => edit_project(id)?,
    };

    Ok(())
}
