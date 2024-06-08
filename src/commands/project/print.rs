use std::io::{self, Write};

use diesel::SqliteConnection;

use crate::data::MdFile;
use crate::models::Project;
use crate::{
    database,
    errors::RemarkError,
    utils::{get_path, RemarkDir},
};

pub(crate) fn print_project(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let project = database::get_project_like(&mut conn, &id)?;
    let project_path = get_path(RemarkDir::Project)?.join(format!("{}.md", project.id));
    let file = MdFile::<Project>::from_file(&project_path)?;

    io::stdout().write_all(file.content.as_bytes())?;

    Ok(())
}
