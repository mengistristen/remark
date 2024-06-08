use std::{
    fs::File,
    io::{self, Read, Write},
};

use diesel::SqliteConnection;

use crate::{
    database::get_report_like,
    errors::RemarkError,
    utils::{get_path, RemarkDir},
};

pub(crate) fn open_report(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let report = get_report_like(&mut conn, &id)?;
    let report_path = get_path(RemarkDir::Report)?.join(format!("{}.md", report.id));
    let mut file = File::open(report_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    io::stdout().write_all(contents.as_bytes())?;

    Ok(())
}
