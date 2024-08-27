use ::std::io::Write;

use diesel::SqliteConnection;

use crate::{
    cli::ReportAction,
    data::MdFile,
    errors::RemarkError,
    models::{Project, Task},
    serializable::SerializableTask,
    utils::{get_path, RemarkDir},
};

use self::{
    generate::generate_report, list::list_reports, open::open_report, print::print_report,
    remove::remove_report,
};

mod generate;
mod list;
mod open;
mod print;
mod remove;

pub(crate) fn output_report<T: Write>(
    mut writer: T,
    task_project_pairs: &Vec<(Task, Project)>,
    report_name: &str,
    exclude_hours: bool,
) -> Result<(), RemarkError> {
    let mut current_date = None;

    writeln!(writer, "# {report_name}\n")?;

    for (task, project) in task_project_pairs {
        if Some(task.date) != current_date {
            writeln!(writer, "---\n")?;
            writeln!(writer, "## {}\n", task.date.format("%A, %-d %B, %C%y"))?;
            current_date = Some(task.date);
        }

        let task_path = get_path(RemarkDir::Task)?.join(format!("{}.md", task.id));
        let md_file = MdFile::<SerializableTask>::from_file(&task_path)?;

        let task_header = if exclude_hours {
            format!("### {} | {}\n", project.name, task.name)
        } else {
            format!(
                "### {} | {} ({} {})\n",
                project.name,
                task.name,
                task.hours,
                if task.hours == 1.0 { "hour" } else { "hours" }
            )
        };

        writer.write(task_header.as_bytes())?;

        writeln!(writer, "{}", md_file.content)?;
    }

    Ok(())
}

pub fn process_report(conn: SqliteConnection, action: ReportAction) -> Result<(), RemarkError> {
    match action {
        ReportAction::Generate {
            name,
            from,
            to,
            tags,
            exclude_hours,
        } => generate_report(conn, name, from, to, tags, exclude_hours)?,
        ReportAction::Open { id } => open_report(conn, id)?,
        ReportAction::Print {
            from,
            to,
            tags,
            exclude_hours,
        } => print_report(conn, from, to, tags, exclude_hours)?,
        ReportAction::Remove { id } => remove_report(conn, id)?,
        ReportAction::List => list_reports(conn)?,
    };

    Ok(())
}
