PRAGMA foreign_keys = OFF;

CREATE TABLE tasks_old (
	id TEXT NOT NULL PRIMARY KEY,
	name TEXT NOT NULL,
	hours FLOAT NOT NULL,
	date DATE NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id)
);

INSERT INTO tasks_old (id, name, hours, date, project_id)
SELECT id, name, hours, date, project_id from tasks;

DROP TABLE tasks;

ALTER TABLE tasks_old RENAME TO tasks;

PRAGMA foreign_keys = ON;
