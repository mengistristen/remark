use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command 
}

#[derive(Subcommand)]
pub enum Command {
    Project {
        #[command(subcommand)]
        command: ProjectCommand
    }
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    Create {
        name: String,
    },
    List
}
