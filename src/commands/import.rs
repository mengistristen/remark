use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Read,
};

use diesel::SqliteConnection;
use flate2::read::GzDecoder;
use tar::{Archive, Entry};

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

    for entry in archive.entries()? {
        if let Err(err) = process_entry(&mut conn, entry?) {
            eprintln!("{}... skipping", err);
        }
    }

    Ok(())
}

fn process_entry(
    conn: &mut SqliteConnection,
    mut entry: Entry<GzDecoder<File>>,
) -> Result<(), RemarkError> {
    let path = entry.header().path()?.into_owned();

    let mut buffer = String::new();
    entry.read_to_string(&mut buffer)?;

    let parent = path.parent().ok_or(RemarkError::Error(format!(
        "error extracting parent for entry {}",
        path.to_string_lossy()
    )))?;
    let file_name = path.file_name().ok_or(RemarkError::Error(format!(
        "error extracting file name for entry {}",
        path.to_string_lossy()
    )))?;

    match parent {
        p if p.ends_with("projects") => process_project(conn, file_name, buffer),
        p if p.ends_with("tasks") => process_task(conn, file_name, buffer),
        p if p.ends_with("reports") => process_report(conn, file_name, buffer),
        _ => Err(RemarkError::Error(format!(
            "invalid parent directory for file {}... skipping",
            path.to_string_lossy()
        ))),
    }?;

    Ok(())
}

fn process_project(
    conn: &mut SqliteConnection,
    file_name: &OsStr,
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
    file_name: &OsStr,
    contents: String,
) -> Result<(), RemarkError> {
    let md_file = MdFile::<SerializableTask>::from_string(contents)?;
    let mut md_path = utils::get_path(RemarkDir::Project)?;

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
    _file_name: &OsStr,
    _contents: String,
) -> Result<(), RemarkError> {
    Err(RemarkError::Error("cannot process reports".to_owned()))
}
