use std::io::{self, Write};

use diesel::SqliteConnection;

use crate::{
    data::MdFile,
    database::get_report_like,
    errors::RemarkError,
    serializable::SerializableReport,
    utils::{get_path, RemarkDir},
};

pub(crate) fn open_report(mut conn: SqliteConnection, id: String) -> Result<(), RemarkError> {
    let report = get_report_like(&mut conn, id.as_str())?;
    let report_path = get_path(RemarkDir::Report)?.join(format!("{}.md", report.id));
    let file = MdFile::<SerializableReport>::from_file(&report_path)?;

    io::stdout().write_all(file.content.as_bytes())?;

    Ok(())
}
