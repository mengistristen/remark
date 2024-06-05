use diesel::prelude::*;

use crate::errors::RemarkError;
use crate::models::{Project, Report, Task};
use crate::schema::projects::{self, dsl as projects_dsl};
use crate::schema::reports::{self, dsl as reports_dsl};
use crate::schema::tasks::{self, dsl as tasks_dsl};

pub(crate) fn insert_project(
    conn: &mut SqliteConnection,
    project: &Project,
) -> Result<(), RemarkError> {
    diesel::insert_into(projects::table)
        .values(project)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn get_projects_like(
    conn: &mut SqliteConnection,
    pattern: String,
) -> Result<Vec<Project>, RemarkError> {
    let result = projects_dsl::projects
        .filter(projects_dsl::id.like(pattern))
        .select(Project::as_select())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn get_all_projects(conn: &mut SqliteConnection) -> Result<Vec<Project>, RemarkError> {
    let result = projects_dsl::projects
        .select(Project::as_select())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn insert_task(conn: &mut SqliteConnection, task: &Task) -> Result<(), RemarkError> {
    diesel::insert_into(tasks::table)
        .values(task)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn get_tasks_like(
    conn: &mut SqliteConnection,
    pattern: String,
) -> Result<Vec<Task>, RemarkError> {
    let result = tasks_dsl::tasks
        .filter(tasks_dsl::id.like(pattern))
        .select(Task::as_select())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn get_staged_tasks(conn: &mut SqliteConnection) -> Result<Vec<Task>, RemarkError> {
    let result = tasks_dsl::tasks
        .select(Task::as_select())
        .filter(tasks_dsl::staged.eq(true))
        .order(tasks_dsl::date.desc())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn get_all_tasks(conn: &mut SqliteConnection) -> Result<Vec<Task>, RemarkError> {
    let result = tasks_dsl::tasks
        .select(Task::as_select())
        .order(tasks_dsl::date.desc())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn mark_task(
    conn: &mut SqliteConnection,
    id: String,
    staged: bool,
) -> Result<(), RemarkError> {
    diesel::update(tasks_dsl::tasks.find(id))
        .set(tasks_dsl::staged.eq(staged))
        .execute(conn)?;

    Ok(())
}

pub(crate) fn insert_report(
    conn: &mut SqliteConnection,
    report: &Report,
) -> Result<(), RemarkError> {
    diesel::insert_into(reports::table)
        .values(report)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn get_reports_like(
    conn: &mut SqliteConnection,
    pattern: String,
) -> Result<Vec<Report>, RemarkError> {
    let result = reports_dsl::reports
        .filter(reports_dsl::id.like(pattern))
        .select(Report::as_select())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn get_all_reports(conn: &mut SqliteConnection) -> Result<Vec<Report>, RemarkError> {
    let result = reports_dsl::reports
        .select(Report::as_select())
        .load(conn)?;

    Ok(result)
}
