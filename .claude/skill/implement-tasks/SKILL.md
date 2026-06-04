---
description: This file describes the skills for Claude to implement the tasks for the Japanese Learn project.
paths:
  - .github/prompts/speckit.tasks.prompt.md
---

# Task completion workflow
After a task is implemented, built, and tested:
1. Suggest a commit message:
   ```
   type: description
   TaskId: Task X.Y
   ```
   where X = phase, Y = task number.
2. Prompt for code review and approval before moving on.
3. If review surfaces improvements or follow-up work: propose the changes, then add approved follow-ups to `@.github/prompts/speckit.tasks.prompt.md` using a dot-suffix number (e.g., `1.3.1`, `1.3.2`) to link them to the original task. Get approval on the updated task list before implementing.