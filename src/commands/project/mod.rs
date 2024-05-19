use crate::ProjectError;
use crate::cli::ProjectCommand;
use crate::commands::project::create::create_list;

pub mod create;
pub mod list;

pub fn process_project(command: ProjectCommand) -> Result<(), ProjectError> {
    match command {
        ProjectCommand::Create { name } => create_list(name)?,
        ProjectCommand::List => {
            println!("listing all projects"); 
            return Ok(())
        },
    };

    Ok(())
}

