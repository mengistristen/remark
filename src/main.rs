use std::fs;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lib_remark::cli::{Cli, Command};
use lib_remark::commands::project::process_project;
use lib_remark::commands::report::process_report;
use lib_remark::commands::task::process_task;
use lib_remark::utils::get_base_dir;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    let base_path = get_base_dir();

    if !base_path.exists() {
        fs::create_dir_all(&base_path).unwrap_or_else(|_| {
            panic!(
                "error: failed to create remark directory at {}",
                base_path.to_string_lossy()
            )
        });
    }

    let database_path = get_base_dir().join("db.sqlite");
    let database_url = format!(
        "sqlite://{}",
        database_path
            .to_str()
            .expect("failed to convert path to string")
    );

    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connecting to {}", database_url));

    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");

    let cli = Cli::parse();
    let conn = SqliteConnection::establish(database_url.as_str())
        .unwrap_or_else(|_| panic!("error connecting to {}", database_url));

    if let Err(err) = match cli.command {
        Command::Project { action } => process_project(conn, action),
        Command::Task { action } => process_task(conn, action),
        Command::Report { action } => process_report(conn, action),
    } {
        eprintln!("{}", err);
    }
}
