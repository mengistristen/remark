// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    reports (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        name -> Text,
        hours -> Float,
        date -> Date,
        project_id -> Text,
    }
}

diesel::table! {
    tags (name) {
        name -> Text,
    }
}

diesel::table! {
    task_tags (task_id, tag_name) {
        task_id -> Text,
        tag_name -> Text,
    }
}

diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(task_tags -> tags (tag_name));

diesel::allow_tables_to_appear_in_same_query!(projects, reports, tasks, tags, task_tags,);
