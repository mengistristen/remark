use renew_lib::cli::{Command, Cli};
use renew_lib::commands::project::process_project;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Project {
            command
        } => process_project(command).unwrap()
    }
}
