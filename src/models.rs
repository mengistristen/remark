use std::borrow::Cow;

use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: String,
    pub name: String,
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

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::reports)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Report {
    pub id: String,
    pub name: String,
}

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Identifiable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: String,
    pub name: String,
    pub hours: f32,
    pub date: chrono::NaiveDate,
    pub project_id: String,
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
