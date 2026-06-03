---
description: This file describes the skills for Claude to implement the tasks for the Japanese Learn project.
paths:
  - .github/prompts/speckit.tasks.prompt.md
---

# General rules:
- read @ .github/prompts/speckit.tasks.prompt.md for the task execution rules and the task breakdown for the Japanese Learn application.
- follow code style and Slint declarative rules defined in @ .claude/rules/slint-code-style.md when writing or reviewing Slint and Rust code.
- after the task is implemented, successfully built, and tested, suggest a commit message following the format: 
```bash
type: description 
TaskId: Task X.Y
```
where X = phase, Y = task number. The commit message should be descriptive of the changes made in the task implementation.
- after suggesting the commit message, prompt for review of the commit codes and approval.
- if there are suggestions for code improvement during the review, or follow up work such as fixing warnings, suggest the necessary changes and implement them before moving on to the next task. Prompt me for follow up tasks.
- if I approve the follow up tasks, edit file @ .github/prompts/speckit.tasks.prompt.md to add the follow up tasks to the task list. Enumerate the follow up tasks with the same task number as the original task, but with a suffix (e.g., 1.3.1, 1.3.2, etc.) to indicate that they are related to the original task. Prompt me for approval of the edited task list before proceeding with the implementation of the next task.