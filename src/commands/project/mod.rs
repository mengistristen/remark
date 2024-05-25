use serde::{Deserialize, Serialize};

use crate::cli::ProjectCommand;
use crate::ProjectError;

use self::create::create_project;
use self::edit::edit_project;
use self::list::list_projects;

pub mod create;
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

pub fn process_project(command: ProjectCommand) -> Result<(), ProjectError> {
    match command {
        ProjectCommand::Create { name } => create_project(name)?,
        ProjectCommand::List => list_projects()?, 
        ProjectCommand::Edit { id } => edit_project(id)?,
    };

    Ok(())
}
