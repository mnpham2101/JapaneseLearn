---
description: Implement the next open task from the task list. Picks the task, delegates to the right agent, verifies the build, and closes with a commit suggestion.
---

# Implement Tasks Skill

When this skill is invoked, execute the following workflow in order.

**Authorship split**: project-owner has already read the codebase and authored the task/subtask files (and, for cross-library tasks, the architecture prompt file) per `task-planning.md` — including agent labels, file lists, and technical approach. This skill governs how task-manager *consumes* that plan and dispatches it; task-manager does not rediscover or redesign it.

## Responsibilities by agent

| Step | Responsible agent | What they do |
|---|---|---|
| 1 — Pick task | **task-manager** | Read the task entry + its already-authored subtask file(s); identify the next open task |
| 2 — Delegate | **task-manager** | Brief the implementing or testing agent by pointing to the authored subtask file, and invoke it |
| 3 — Build/test verification | **slint-developer**, **rust-developer**, or **slint-tester** | Run `cargo build` or test suite; report result before returning |
| 4a — Suggest commit | **slint-developer** or **rust-developer** | Suggest commit message per `commit-msg-format.md`; stop there |
| 4b — Review gate, commit, mark done | **task-manager** | Prompt for review, commit after approval, mark task `[x]` |

Implementing agents (slint-developer, rust-developer) handle Steps 3 and 4a. **slint-tester** handles Step 3 only — runs tests, reports verdict, does not commit. Steps 1, 2, and 4b are always task-manager's responsibility. No agent other than task-manager prompts for review or marks tasks done.

---

## Step 1 — Pick the task  `[task-manager]`

Read `@.github/prompts/speckit.tasks.prompt.md`. Find the first unchecked task (`- [ ]`) in the current phase, and open its `speckit.subtask.M-N-X.prompt.md` file(s) — project-owner has already written the technical approach, file list, and agent label there. If the user specified a task ID, use that instead.

State the task ID and description before doing anything else. If the authored plan is missing or looks incomplete for this task, send it back to **project-owner** to author rather than filling the gap yourself — do not improvise a plan task-manager wasn't meant to design.

## Step 2 — Determine the agent and delegate  `[task-manager]`

The agent label is already on the task/subtask — read it, don't re-derive it:
- Task labelled `**[slint-developer]**` → invoke the **slint-developer** agent.
- Task labelled `**[rust-developer]**` → invoke the **rust-developer** agent.
- Task labelled `**[slint-tester]**` → invoke the **slint-tester** agent.
- No label, or a verification/manual task → handle directly without delegating.

Brief the agent with: the goal (one sentence, from the subtask file), a pointer to its `speckit.subtask.M-N-X.prompt.md` (the technical approach — files to change, components, functions/callbacks, patterns — is already written there), and any constraint the agent might not already know. Do not copy file contents, and do not restate the technical approach the subtask file already contains — point to it.

## Step 3 — Build verification  `[slint-developer / rust-developer]`

The implementing agent runs this as part of its own non-negotiable rules. Task-manager trusts the agent's build report for single-agent tasks. Run `cargo build` independently only when chaining multiple agents (handoff verification):

```powershell
cargo build --bin japanese_learn
```

If the build fails with `LNK1201` (PDB locked):
```powershell
taskkill /F /IM japanese_learn.exe 2>$null
Remove-Item "target\debug\deps\japanese_learn.pdb" -ErrorAction SilentlyContinue
Remove-Item "target\debug\japanese_learn.pdb" -ErrorAction SilentlyContinue
cargo build --bin japanese_learn
```

Do not proceed to Step 4 until the build is green.

## Step 4 — Commit workflow

### 4a — Suggest commit message  `[slint-developer / rust-developer]`

The implementing agent suggests the commit message at the end of its session, following `@.claude/rules/commit-msg-format.md`. The agent stops after suggesting — it does not prompt for review, commit, or mark the task done.

### 4b — Review gate, commit, mark done  `[task-manager]`

1. Present the agent's commit message suggestion to the user. Prompt for code review and approval before committing, unless the user has said not to.

2. After approval, run:
   ```powershell
   cargo fmt
   cargo clippy
   git add <changed files>
   git commit -m "<message>"
   ```

3. Mark the task done in `@.github/prompts/speckit.tasks.prompt.md`: change `- [ ]` to `- [x]`.

4. If review surfaces follow-up work: propose the changes, add approved follow-ups using dot-suffix numbers (e.g., `1.3.1`), and get approval before implementing them.
