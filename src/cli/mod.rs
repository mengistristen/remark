use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    Report {
        #[command(subcommand)]
        action: ReportAction,
    }
}

#[derive(Subcommand)]
pub enum ProjectAction {
    Add { name: String },
    List,
    Edit { id: String },
}

#[derive(Subcommand)]
pub enum TaskAction {
   Add,
   List,
   Edit,
   Stage, 
   UnStage
}

#[derive(Subcommand)]
pub enum ReportAction {
    Add
}
