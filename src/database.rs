use diesel::prelude::*;

use crate::errors::RemarkError;
use crate::models::{Project, Report, Task, TaskTag};
use crate::schema::projects::{self, dsl as projects_dsl};
use crate::schema::reports::{self, dsl as reports_dsl};
use crate::schema::task_tags::{self, dsl as task_tags_dsl};
use crate::schema::tasks::{self, dsl as tasks_dsl};
use crate::serializable::{UpdateProject, UpdateTask};

pub(crate) fn insert_project(
    conn: &mut SqliteConnection,
    project: &Project,
) -> Result<(), RemarkError> {
    diesel::insert_into(projects::table)
        .values(project)
        .execute(conn)?;

    Ok(())
}

pub(crate) fn update_project(
    conn: &mut SqliteConnection,
    id: String,
    project: &UpdateProject,
) -> Result<Project, RemarkError> {
    diesel::update(projects_dsl::projects.filter(projects_dsl::id.eq(&id)))
        .set(projects_dsl::name.eq(&project.name))
        .execute(conn)?;

    let project = get_project_like(conn, &id)?;

    Ok(project)
}

pub(crate) fn remove_project(conn: &mut SqliteConnection, id: &String) -> Result<(), RemarkError> {
    diesel::delete(projects_dsl::projects.filter(projects_dsl::id.eq(id))).execute(conn)?;

    Ok(())
}

pub(crate) fn get_project_like(
    conn: &mut SqliteConnection,
    begins_with: &String,
) -> Result<Project, RemarkError> {
    let mut projects = get_projects_like(conn, begins_with)?;

    if projects.is_empty() {
        return Err(RemarkError::Error(format!(
            "failed to find a project beginning with '{begins_with}'"
        )));
    }

    if projects.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one project beginning with '{begins_with}'"
        )));
    }

    Ok(projects.remove(0))
}

pub(crate) fn get_projects_like(
    conn: &mut SqliteConnection,
    begins_with: &String,
) -> Result<Vec<Project>, RemarkError> {
    let pattern = format!("{}%", begins_with);
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

pub(crate) fn insert_task(
    conn: &mut SqliteConnection,
    task: &Task,
    tags: Option<&[String]>,
) -> Result<(), RemarkError> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        diesel::insert_into(tasks::table)
            .values(task)
            .execute(conn)?;

        if let Some(tags) = tags {
            let task_tags: Vec<TaskTag> = tags
                .iter()
                .map(|tag| TaskTag {
                    task_id: task.id.clone(),
                    tag_name: tag.clone(),
                })
                .collect();

            diesel::insert_into(task_tags::table)
                .values(task_tags)
                .execute(conn)?;
        }

        Ok(())
    })?;

    Ok(())
}

pub(crate) fn update_task(
    conn: &mut SqliteConnection,
    id: &String,
    task: &UpdateTask,
) -> Result<Task, RemarkError> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        diesel::update(tasks_dsl::tasks.filter(tasks_dsl::id.eq(&id)))
            .set((
                tasks_dsl::name.eq(&task.name),
                tasks_dsl::hours.eq(&task.hours),
                tasks_dsl::date.eq(&task.date),
            ))
            .execute(conn)?;

        diesel::delete(task_tags_dsl::task_tags.filter(task_tags_dsl::task_id.eq(id)))
            .execute(conn)?;

        if let Some(ref tags) = task.tags {
            let task_tags: Vec<TaskTag> = tags
                .iter()
                .map(|tag| TaskTag {
                    task_id: id.clone(),
                    tag_name: tag.clone(),
                })
                .collect();

            diesel::insert_into(task_tags::table)
                .values(task_tags)
                .execute(conn)?;
        }

        Ok(())
    })?;

    let task = get_task_like(conn, id)?;

    Ok(task)
}

pub(crate) fn remove_task(conn: &mut SqliteConnection, id: &String) -> Result<(), RemarkError> {
    diesel::delete(tasks_dsl::tasks.filter(tasks_dsl::id.eq(id))).execute(conn)?;

    Ok(())
}

pub(crate) fn remove_tasks_for_project(
    conn: &mut SqliteConnection,
    project_id: &String,
) -> Result<(), RemarkError> {
    diesel::delete(tasks_dsl::tasks.filter(tasks_dsl::project_id.eq(project_id))).execute(conn)?;

    Ok(())
}

pub(crate) fn get_tasks_for_project(
    conn: &mut SqliteConnection,
    project_id: &String,
) -> Result<Vec<Task>, RemarkError> {
    let tasks = tasks_dsl::tasks
        .filter(tasks_dsl::project_id.eq(project_id))
        .order(tasks_dsl::date.asc())
        .load(conn)?;

    Ok(tasks)
}

pub(crate) fn get_task_like(
    conn: &mut SqliteConnection,
    begins_with: &String,
) -> Result<Task, RemarkError> {
    let mut tasks = get_tasks_like(conn, begins_with)?;

    if tasks.is_empty() {
        return Err(RemarkError::Error(format!(
            "failed to find a task beginning with '{begins_with}'"
        )));
    }

    if tasks.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one task beginning with '{begins_with}'"
        )));
    }

    Ok(tasks.remove(0))
}

pub(crate) fn get_tasks_like(
    conn: &mut SqliteConnection,
    begins_with: &String,
) -> Result<Vec<Task>, RemarkError> {
    let pattern = format!("{}%", begins_with);
    let result = tasks_dsl::tasks
        .filter(tasks_dsl::id.like(pattern))
        .select(Task::as_select())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn get_tasks_in_range(
    conn: &mut SqliteConnection,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
    tags: Option<Vec<String>>,
) -> Result<Vec<(Task, Project)>, RemarkError> {
    let mut query = tasks::table
        .inner_join(projects::table)
        .left_join(task_tags::table.on(task_tags_dsl::task_id.eq(tasks_dsl::id)))
        .into_boxed();

    if let Some(tags) = tags {
        query = query.filter(task_tags_dsl::tag_name.eq_any(tags));
    }

    let query = query
        .filter(tasks_dsl::date.ge(from))
        .filter(tasks_dsl::date.le(to))
        .order(tasks_dsl::date.asc())
        .select((Task::as_select(), Project::as_select()));

    let tasks_with_projects = query.load::<(Task, Project)>(conn)?;

    Ok(tasks_with_projects)
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

pub(crate) fn remove_report(conn: &mut SqliteConnection, id: &String) -> Result<(), RemarkError> {
    diesel::delete(reports_dsl::reports.filter(reports_dsl::id.eq(id))).execute(conn)?;

    Ok(())
}

pub(crate) fn get_report_like(
    conn: &mut SqliteConnection,
    begins_with: &String,
) -> Result<Report, RemarkError> {
    let mut reports = get_reports_like(conn, begins_with)?;

    if reports.is_empty() {
        return Err(RemarkError::Error(format!(
            "failed to find a report beginning with '{begins_with}'"
        )));
    }

    if reports.len() != 1 {
        return Err(RemarkError::Error(format!(
            "found more than one report beginning with '{begins_with}'"
        )));
    }

    Ok(reports.remove(0))
}

pub(crate) fn get_reports_like(
    conn: &mut SqliteConnection,
    begins_with: &String,
) -> Result<Vec<Report>, RemarkError> {
    let pattern = format!("{}%", begins_with);
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

pub(crate) fn get_tags_for_task(
    conn: &mut SqliteConnection,
    task_id: &String,
) -> Result<Option<Vec<String>>, RemarkError> {
    let result: Vec<String> = task_tags_dsl::task_tags
        .filter(task_tags_dsl::task_id.eq(task_id))
        .select(task_tags_dsl::tag_name)
        .load(conn)?;

    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}
