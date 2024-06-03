use crate::models::Project;
use crate::schema::projects::dsl::*;
use diesel::prelude::*;

use crate::errors::RemarkError;

pub(crate) fn list_projects(mut conn: SqliteConnection) -> Result<(), RemarkError> {
    let result = projects.select(Project::as_select()).load(&mut conn)?;

    for project in result {
        println!("{} {}", project.id, project.name);
    }

    Ok(())
}
