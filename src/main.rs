use clap::Parser;
use diesel::{Connection, SqliteConnection};
use lib_remark::cli::{Cli, Command};
use lib_remark::commands::project::process_project;
use lib_remark::commands::task::process_task;
use lib_remark::errors::RemarkError;
use lib_remark::utils::get_base_dir;

fn main() -> Result<(), RemarkError> {
    let cli = Cli::parse();

    let database_url = get_base_dir().join("db.sqlite");
    let database_url = database_url
        .to_str()
        .expect("failed to convert path to string");
    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    match cli.command {
        Command::Project { action } => process_project(conn, action)?,
        Command::Task { action } => process_task(conn, action)?,
        Command::Report { action: _ } => todo!(),
    };

    Ok(())
}
