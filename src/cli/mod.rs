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
        #[arg(long)]
        from: chrono::NaiveDate,
        #[arg(long)]
        to: Option<chrono::NaiveDate>,
    },
    /// Edit a task
    Edit {
        #[arg(long)]
        id: String,
        #[arg(short, long, default_value_t = false)]
        metadata: bool,
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
        name: Option<String>,
        #[arg(long)]
        from: chrono::NaiveDate,
        #[arg(long)]
        to: Option<chrono::NaiveDate>,
    },
    Open {
        #[arg(long)]
        id: String,
    },
    Print {
        #[arg(long)]
        from: chrono::NaiveDate,
        #[arg(long)]
        to: Option<chrono::NaiveDate>,
    },
    List,
}
