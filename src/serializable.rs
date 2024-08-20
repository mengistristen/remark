use std::borrow::Cow;

use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::{
    database,
    errors::RemarkError,
    models::{Project, Task},
};

#[derive(Serialize, Deserialize)]
pub struct SerializableProject<'a> {
    pub id: Cow<'a, String>,
    pub name: Cow<'a, String>,
}

impl<'a> From<&'a Project> for SerializableProject<'a> {
    fn from(project: &'a Project) -> Self {
        Self {
            id: Cow::Borrowed(&project.id),
            name: Cow::Borrowed(&project.name),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableTask<'a> {
    pub id: Cow<'a, String>,
    pub name: Cow<'a, String>,
    pub hours: Cow<'a, f32>,
    pub date: Cow<'a, chrono::NaiveDate>,
    pub project_id: Cow<'a, String>,
    pub tags: Option<Cow<'a, Vec<String>>>,
}

impl<'a> SerializableTask<'a> {
    pub(crate) fn from_task(
        conn: &mut SqliteConnection,
        task: &'a Task,
    ) -> Result<Self, RemarkError> {
        let tags = database::get_tags_for_task(conn, &task.id)?;

        Ok(Self {
            id: Cow::Borrowed(&task.id),
            name: Cow::Borrowed(&task.name),
            hours: Cow::Borrowed(&task.hours),
            date: Cow::Borrowed(&task.date),
            project_id: Cow::Borrowed(&task.project_id),
            tags: match tags {
                Some(tags) => Some(Cow::Owned(tags)),
                None => None,
            },
        })
    }

    pub(crate) fn from_task_with_tags(task: &'a Task, tags: Option<&'a Vec<String>>) -> Self {
        Self {
            id: Cow::Borrowed(&task.id),
            name: Cow::Borrowed(&task.name),
            hours: Cow::Borrowed(&task.hours),
            date: Cow::Borrowed(&task.date),
            project_id: Cow::Borrowed(&task.project_id),
            tags: match tags {
                Some(tags) => Some(Cow::Borrowed(tags)),
                None => None,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProject<'a> {
    pub name: Cow<'a, String>,
}

impl<'a> UpdateProject<'a> {
    pub fn from_project(project: &'a Project) -> Self {
        Self {
            name: Cow::Borrowed(&project.name),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTask<'a> {
    pub name: Cow<'a, String>,
    pub hours: Cow<'a, f32>,
    pub date: Cow<'a, chrono::NaiveDate>,
    pub tags: Option<Cow<'a, Vec<String>>>,
}

impl<'a> UpdateTask<'a> {
    pub fn from_task(conn: &mut SqliteConnection, task: &'a Task) -> Result<Self, RemarkError> {
        let tags = database::get_tags_for_task(conn, &task.id)?;

        Ok(Self {
            name: Cow::Borrowed(&task.name),
            hours: Cow::Borrowed(&task.hours),
            date: Cow::Borrowed(&task.date),
            tags: match tags {
                Some(tags) => Some(Cow::Owned(tags)),
                None => None,
            },
        })
    }
}
