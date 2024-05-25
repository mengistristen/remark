use clap::Parser;
use lib_remark::cli::{Cli, Command};
use lib_remark::commands::project::process_project;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Project { command } => process_project(command).unwrap(),
    }
}
