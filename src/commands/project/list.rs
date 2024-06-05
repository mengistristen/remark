use diesel::SqliteConnection;

use crate::database;

use crate::errors::RemarkError;

pub(crate) fn list_projects(mut conn: SqliteConnection) -> Result<(), RemarkError> {
    let result = database::get_all_projects(&mut conn)?;

    for project in result {
        println!("{} {}", project.id, project.name);
    }

    Ok(())
}
