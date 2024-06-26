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

diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(projects, reports, tasks,);
