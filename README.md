# Remark

A command line tool for tracking work done on projects using Markdown.

## Usage

`remark project add <name>` - creates a new project
`remark project edit <id>` - edits a project
`remark project list` - lists the ids and names of all projects
`remark task add <name>` - creates a new task
`remark task edit <id>` - edits a task
`remark task list --all` - lists staged tasks, optionally all
`remark task stage` - marks a task as not having been reported
`remark task unstage` - marks a task as having been reported
`remark report add <name>` - creates a new report, marking staged tasks as having been reported 
