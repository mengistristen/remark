use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::models::{Project, Task};

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
}

impl<'a> From<&'a Task> for SerializableTask<'a> {
    fn from(value: &'a Task) -> Self {
        Self {
            id: Cow::Borrowed(&value.id),
            name: Cow::Borrowed(&value.name),
            hours: Cow::Borrowed(&value.hours),
            date: Cow::Borrowed(&value.date),
            project_id: Cow::Borrowed(&value.project_id),
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
}

impl<'a> UpdateTask<'a> {
    pub fn from_task(task: &'a Task) -> Self {
        Self {
            name: Cow::Borrowed(&task.name),
            hours: Cow::Borrowed(&task.hours),
            date: Cow::Borrowed(&task.date),
        }
    }
}
