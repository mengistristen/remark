use ::std::io::Write;

use diesel::SqliteConnection;

use crate::{
    cli::ReportAction,
    data::MdFile,
    errors::RemarkError,
    models::Task,
    utils::{get_path, DataDir},
};

use self::{generate::generate_report, list::list_reports, print::print_report};

mod generate;
mod list;
mod print;

pub(crate) fn output_report<T: Write>(
    mut writer: T,
    tasks: &Vec<Task>,
    report_name: &String,
) -> Result<(), RemarkError> {
    let mut current_date = None;

    writeln!(writer, "# {report_name}\n")?;

    for task in tasks {
        if Some(task.date) != current_date {
            writeln!(writer, "## {}\n", task.date.format("%A, %-d %B, %C%y"))?;
            current_date = Some(task.date);
        }

        let task_path = get_path(DataDir::Task)?.join(format!("{}.md", task.id));
        let md_file = MdFile::<Task>::from_file(&task_path)?;

        writeln!(writer, "### {} ({} hours)\n", task.name, task.hours)?;

        writeln!(writer, "{}", md_file.content)?;
    }

    Ok(())
}

pub fn process_report(conn: SqliteConnection, action: ReportAction) -> Result<(), RemarkError> {
    match action {
        ReportAction::Generate { name, skip_marking } => generate_report(conn, name, skip_marking)?,
        ReportAction::Print { id } => print_report(conn, id)?,
        ReportAction::List => list_reports(conn)?,
    };

    Ok(())
}
