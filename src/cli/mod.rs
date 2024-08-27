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
    /// Import data
    Import {
        #[arg(short, long)]
        input_file: String,
    },
    /// Export data
    Export {
        #[arg(short, long)]
        output_file: String,
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
    /// Print a project
    Print {
        #[arg(long)]
        id: String,
        #[arg(long, default_value_t = false)]
        include_tasks: bool,
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
        from: Option<chrono::NaiveDate>,
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
    /// Print a task
    Print {
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ReportAction {
    /// Create a new report
    Generate {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        from: chrono::NaiveDate,
        #[arg(long)]
        to: Option<chrono::NaiveDate>,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        #[arg(long)]
        exclude_hours: bool,
    },
    /// Print a report by ID
    Open {
        #[arg(long)]
        id: String,
    },
    /// Print a report based on the tasks in the given date range
    Print {
        #[arg(long)]
        from: Option<chrono::NaiveDate>,
        #[arg(long)]
        to: Option<chrono::NaiveDate>,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        #[arg(long)]
        exclude_hours: bool,
    },
    /// Remove a report
    Remove {
        #[arg(long)]
        id: String,
    },
    /// List all reports
    List,
}
