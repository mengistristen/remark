use std::io::{self, Write};

use diesel::SqliteConnection;

use crate::data::MdFile;
use crate::models::{Project, Task};
use crate::{
    database,
    errors::RemarkError,
    utils::{get_path, RemarkDir},
};

pub(crate) fn print_project(mut conn: SqliteConnection, id: String, include_tasks: bool) -> Result<(), RemarkError> {
    let project = database::get_project_like(&mut conn, &id)?;
    let project_path = get_path(RemarkDir::Project)?.join(format!("{}.md", project.id));
    let file = MdFile::<Project>::from_file(&project_path)?;
    let mut stdout = io::stdout();

    stdout.write_all(file.content.as_bytes())?;

    if include_tasks {
        let mut current_date = None;
        let tasks = database::get_tasks_for_project(&mut conn, &project.id)?;

        write!(stdout, "\n")?;

        for task in tasks {
            let task_path = get_path(RemarkDir::Task)?.join(format!("{}.md", task.id));
            if Some(task.date) != current_date {
                writeln!(stdout, "---\n")?;
                writeln!(stdout, "## {}\n", task.date.format("%A, %-d %B, %C%y"))?;
                current_date = Some(task.date);
            }

            writeln!(stdout, "### {}\n", task.name)?;

            let md_file = MdFile::<Task>::from_file(&task_path)?;

            stdout.write_all(md_file.content.as_bytes())?;

            write!(stdout, "\n")?;
        }
    }

    Ok(())
}
