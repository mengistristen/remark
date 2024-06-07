-- Your SQL goes here
CREATE TABLE `reports`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

CREATE TABLE `projects`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

CREATE TABLE `tasks`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`hours` FLOAT NOT NULL,
	`date` DATE NOT NULL,
    `project_id` TEXT NOT NULL REFERENCES projects(id)
);
