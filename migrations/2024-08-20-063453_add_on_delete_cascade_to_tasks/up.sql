PRAGMA foreign_keys = OFF;

CREATE TABLE tasks_new (
	id TEXT NOT NULL PRIMARY KEY,
	name TEXT NOT NULL,
	hours FLOAT NOT NULL,
	date DATE NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE
);

INSERT INTO tasks_new (id, name, hours, date, project_id)
SELECT id, name, hours, date, project_id from tasks;

DROP TABLE tasks;

ALTER TABLE tasks_new RENAME TO tasks;

PRAGMA foreign_keys = ON;
