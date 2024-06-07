-- This file should undo anything in `up.sql`


ALTER TABLE `tasks` ADD COLUMN `staged` BOOL NOT NULL;

