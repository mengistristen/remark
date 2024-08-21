use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    Selectable,
};

use crate::serializable::{SerializableProject, SerializableTask};

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: String,
    pub name: String,
}

impl From<SerializableProject<'_>> for Project {
    fn from(value: SerializableProject<'_>) -> Self {
        Self {
            id: value.id.into_owned(),
            name: value.name.into_owned(),
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

#[derive(Queryable, Selectable, Insertable, Associations, Identifiable, Debug, Clone)]
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

impl From<SerializableTask<'_>> for Task {
    fn from(value: SerializableTask<'_>) -> Self {
        Self {
            id: value.id.into_owned(),
            name: value.name.into_owned(),
            hours: value.hours.into_owned(),
            date: value.date.into_owned(),
            project_id: value.project_id.into_owned(),
        }
    }
}

#[derive(Insertable, Associations)]
#[diesel(table_name = crate::schema::task_tags)]
#[diesel(belongs_to(Task))]
pub struct TaskTag {
    pub task_id: String,
    pub tag_name: String,
}
