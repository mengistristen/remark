use diesel::SqliteConnection;

use crate::errors::RemarkError;

pub fn process_import(conn: SqliteConnection, input_file: String) -> Result<(), RemarkError> {
    todo!()
}
