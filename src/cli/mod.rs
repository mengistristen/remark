use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage projects
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
    /// Manage tasks
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    /// Manage reports
    Report {
        #[command(subcommand)]
        action: ReportAction,
    },
}

#[derive(Subcommand)]
pub enum ProjectAction {
    /// Create a new project
    Add,
    /// List all projects
    List,
    /// Edit a project
    Edit {
        #[arg(long)]
        id: String,
        #[arg(short, long, default_value_t = false)]
        metadata: bool,
    },
    /// Remove a project
    Remove {
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum TaskAction {
    /// Create a new task
    Add {
        #[arg(long)]
        project: String,
    },
    /// List all tasks
    List {
        #[arg(short, long, default_value_t = false)]
        staged: bool,
    },
    /// Edit a task
    Edit {
        #[arg(long)]
        id: String,
        #[arg(short, long, default_value_t = false)]
        metadata: bool,
    },
    /// Stage a task for being used in a report
    Stage {
        #[arg(long)]
        id: String,
    },
    /// UnStage a task
    UnStage {
        #[arg(long)]
        id: String,
    },
    /// Remove a task
    Remove {
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ReportAction {
    /// Creates a new report
    Generate {
        #[arg(long)]
        name: String,
        #[arg(short, long, default_value_t = false)]
        skip_marking: bool,
    },
    Print {
        #[arg(long)]
        id: Option<String>,
    },
    List,
}
