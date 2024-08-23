use std::fs::{self, File};
use std::io::Read;

use diesel::SqliteConnection;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::{
    data::MdFile,
    database,
    errors::RemarkError,
    models::{Project, Task},
    serializable::{SerializableProject, SerializableTask},
    utils::{self, RemarkDir},
};

pub fn process_import(mut conn: SqliteConnection, input_file: String) -> Result<(), RemarkError> {
    let file = File::open(input_file)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let mut projects = vec![];
    let mut tasks = vec![];
    let mut reports = vec![];

    for entry in archive.entries()? {
        let mut entry = entry?;

        let path = match entry.header().path() {
            Ok(p) => p.into_owned(),
            Err(err) => {
                eprintln!("error extracting path: {}... skipping", err);
                continue;
            }
        };

        let parent = match path.parent() {
            Some(p) => p,
            None => {
                eprintln!(
                    "error extracting parent for entry {}... skipping",
                    path.to_string_lossy()
                );
                continue;
            }
        };

        let file_name = match path.file_name() {
            Some(f) => f.to_string_lossy().into_owned(),
            None => {
                eprintln!(
                    "error extracting file name for entry {}... skipping",
                    path.to_string_lossy()
                );
                continue;
            }
        };

        // sadly it seems I have to read all the files into memory, seems reading
        // the entries out of order causes some corruption
        let mut contents = String::new();
        entry.read_to_string(&mut contents)?;

        match parent {
            p if p.ends_with("projects") => projects.push((file_name, contents)),
            p if p.ends_with("tasks") => tasks.push((file_name, contents)),
            p if p.ends_with("reports") => reports.push((file_name, contents)),
            _ => eprintln!(
                "invalid parent directory for entry {}... skipping",
                path.to_string_lossy()
            ),
        };
    }

    for entry in projects {
        if let Err(err) = process_project(&mut conn, &entry.0, entry.1) {
            eprintln!("project {}: {}... skipping", entry.0, err);
        }
    }

    for entry in tasks {
        if let Err(err) = process_task(&mut conn, &entry.0, entry.1) {
            eprintln!("task {}: {}... skipping", entry.0, err);
        }
    }

    for entry in reports {
        if let Err(err) = process_report(&mut conn, &entry.0, entry.1) {
            eprintln!("report {}: {}... skipping", entry.0, err);
        }
    }

    Ok(())
}

fn process_project(
    conn: &mut SqliteConnection,
    file_name: &str,
    contents: String,
) -> Result<(), RemarkError> {
    let md_file = MdFile::<SerializableProject>::from_string(contents)?;
    let mut md_path = utils::get_path(RemarkDir::Project)?;

    md_path.push(file_name);
    md_file.save(&md_path)?;

    let project: Project = md_file.metadata.into();
    let result = database::insert_project(conn, &project);

    if result.is_err() {
        fs::remove_file(md_path)?;
    }

    result
}

fn process_task(
    conn: &mut SqliteConnection,
    file_name: &str,
    contents: String,
) -> Result<(), RemarkError> {
    let md_file = MdFile::<SerializableTask>::from_string(contents)?;
    let mut md_path = utils::get_path(RemarkDir::Task)?;

    md_path.push(file_name);
    md_file.save(&md_path)?;

    let mut tags = vec![];

    if let Some(ref task_tags) = md_file.metadata.tags {
        for tag in task_tags.iter() {
            tags.push(tag.to_owned());
        }
    }

    let task: Task = md_file.metadata.into();
    let result = database::insert_task(
        conn,
        &task,
        if tags.is_empty() { None } else { Some(&tags) },
    );

    if result.is_err() {
        fs::remove_file(md_path)?;
    }

    result
}

fn process_report(
    _conn: &mut SqliteConnection,
    _file_name: &str,
    _contents: String,
) -> Result<(), RemarkError> {
    Err(RemarkError::Error("cannot process reports".to_owned()))
}
