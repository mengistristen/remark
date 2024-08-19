CREATE TABLE task_tags (
    task_id TEXT NOT NULL,
    tag_name TEXT NOT NULL,
    PRIMARY KEY (task_id, tag_name),
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (tag_name) REFERENCES tags(name)
);
