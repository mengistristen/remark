use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    Selectable,
};

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: String,
    pub name: String,
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

#[derive(Queryable)]
pub struct Tag {
    pub name: String,
}

#[derive(Insertable, Associations)]
#[diesel(table_name = crate::schema::task_tags)]
#[diesel(belongs_to(Task))]
#[diesel(belongs_to(Tag, foreign_key = tag_name))]
pub struct TaskTag {
    pub task_id: String,
    pub tag_name: String,
}
