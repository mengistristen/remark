use crate::data::MdFile;
use crate::models::{Report, Task};
use crate::schema::reports;
use crate::schema::tasks::dsl::*;
use crate::utils::{get_path, DataDir};
use diesel::prelude::*;
use std::fs::{self};
use std::io::Write;
use uuid::Uuid;

use crate::errors::RemarkError;

pub(crate) fn generate_report(
    mut conn: SqliteConnection,
    report_name: String,
    skip_marking: bool,
) -> Result<(), RemarkError> {
    let report_id = Uuid::new_v4();
    let path = get_path(DataDir::Report)?.join(format!("{}.md", report_id));
    let result = tasks
        .select(Task::as_select())
        .filter(staged.eq(true))
        .order(date.desc())
        .load(&mut conn)?;

    if result.len() == 0 {
        return Err(RemarkError::Error(
            "cannot create a report with no tasks".to_owned(),
        ));
    }

    // create the report file in a new scope so that it is closed
    // before the program might try to remove it
    {
        let mut report_file = fs::File::create_new(path.clone())?;
        let mut current_date = None;

        writeln!(report_file, "# {report_name}\n")?;

        for task in result {
            if Some(task.date) != current_date {
                writeln!(report_file, "## {}\n", task.date)?;
                current_date = Some(task.date);
            }

            let task_path = get_path(DataDir::Task)?.join(format!("{}.md", task.id));
            let md_file = MdFile::<Task>::from_file(&task_path)?;

            writeln!(report_file, "### {} ({} hours)\n", task.name, task.hours)?;

            writeln!(report_file, "{}", md_file.content)?;

            if !skip_marking {
                diesel::update(tasks.find(task.id))
                    .set(staged.eq(false))
                    .execute(&mut conn)?;
            }
        }
    }

    let report = Report {
        id: report_id.to_string(),
        name: report_name,
    };

    if let Err(err) = diesel::insert_into(reports::table)
        .values(report)
        .execute(&mut conn)
    {
        fs::remove_file(path)?;
        return Err(err.into());
    }

    println!("generated report '{}'", report_id);

    Ok(())
}
