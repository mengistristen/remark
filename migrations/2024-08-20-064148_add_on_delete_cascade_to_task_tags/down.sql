PRAGMA foreign_keys = OFF;

CREATE TABLE task_tags_old (
    task_id TEXT NOT NULL,
    tag_name TEXT NOT NULL,
    PRIMARY KEY (task_id, tag_name),
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);

INSERT INTO task_tags_old (task_id, tag_name)
SELECT task_id, tag_name FROM task_tags;

DROP TABLE task_tags;

ALTER TABLE task_tags_old RENAME TO task_tags;

PRAGMA foreign_keys = ON;