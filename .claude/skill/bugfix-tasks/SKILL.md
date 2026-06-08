---
description: Pick up an open bug from the bug list, confirm it, plan the fix, delegate implementation, and close it out with review and commit.
---

# Bugfix Tasks Skill

When this skill is invoked, execute the following workflow in order.

## BugID format

A BugID is `X.Y` — `X` is the phase number, `Y` is the sequence number within that phase (e.g., `6.5`). The bug list lives under "## Bug lists" in `@.github/prompts/speckit.specify.prompt.md`. When a BugID appears in a filename, replace the dot with a dash: the report for `6.5` is `@.github/speckit.bug.6-5.report.md`.

## Responsibilities by agent

| Step | Responsible agent | What they do |
|---|---|---|
| 1 — Pick bug | **task-manager** | Read the bug list; select the next "(not done)" bug, or re-open one the user reports as regressed |
| 2 — Confirm | **task-manager** | Reproduce/analyze, get user confirmation, write the findings into the bug report file, commit `Confirm bug [BugID]` |
| 3 — Plan fix | **task-manager** | Write the fix plan into the bug report file, break it into tasks/subtasks, get approval, commit `Solution plan [BugID]` |
| 4 — Delegate | **task-manager** | Brief and invoke the implementing agent(s) |
| 5 — Implement | **slint-developer** / **rust-developer** | Implement the fix, build, suggest a commit message |
| 6 — Test | **slint-tester** | Add or update CRUD/unit tests covering the fix, where the bug's behavior is testable |
| 7 — Close | **task-manager** | Review gate, commit, mark the bug `(done)` |

Steps 4, 5, 7 follow the same agent boundaries as `implement-tasks/SKILL.md` Steps 2–4: only task-manager delegates, reviews, commits, and marks work done; implementing agents build and suggest commit messages, then stop. Step 6 follows `testing-tasks/SKILL.md` Approach 1.

## Step 1 — Pick the bug `[task-manager]`

Read the "## Bug lists" section of `@.github/prompts/speckit.specify.prompt.md`. Select the first bug marked `(not done)`. If the user reports a `(done)` bug has regressed, re-open it (change its status back to `(not done)`) and select it instead.

## Step 2 — Confirm the bug `[task-manager]`

1. If the bug entry references a report file (`@.github/speckit.bug.[BugID].report.md`), read it for the description and any suggested fix. If no such file exists yet, you will create it in step 6.
2. Read the relevant code; assess any suggested fix for feasibility and refine it.
3. **UI bugs**: run the app, capture a screenshot, and compare it against the reported symptom before concluding your analysis.
4. **Backend bugs**: compare the bug description against the expected behavior; read the code that is supposed to produce that behavior and analyze where it diverges.
5. **Slint reactivity/codegen bugs** (a property or binding silently doesn't update the UI): when source-level inspection of the `.slint`/`.rs` is inconclusive, compare the *generated* Rust for the broken component against a similar *working* one. Build, locate the emitted file (`target/debug/build/<crate>-*/out/*.rs`), and diff the two for the property/binding in question — e.g. whether the compiler emitted `set_property_binding(...)` (a real reactive `Property` cell) or constant-folded the read away entirely (`ModelRc::new(VecModel::from(vec![]))` with no reference to the global). To capture both states for the same crate: copy the current generated file aside, edit the source, `touch` it to force regeneration, rebuild, diff, then restore the source and rebuild again. This pinpoints the exact codegen divergence — and thus the root cause — far faster than testing theories about timing, caching, or duplicate globals (see Bug 6.4: a property declared without an explicit binding was const-folded to `[]` at every read site, while the working analogue's explicit binding produced a genuine reactive property read).
6. Present your findings — confirmed symptom, root cause, evidence (screenshot or code excerpt) — to the user and get confirmation that this is the right bug and root cause.
7. **Write these findings into `@.github/speckit.bug.[BugID].report.md`** (create the file if it doesn't exist): bug description, confirmed symptom, root cause, and supporting evidence. This file is the source of truth the rest of the workflow builds on.
8. Commit the report file with message `Confirm bug [BugID]`.

### Variant — CRITICAL bug, dependency issue, or version conflict `[task-manager]`

Use this in place of the steps above when the bug is a build failure, dependency conflict, or version mismatch rather than an application-behavior bug:
1. Read the error output or symptom carefully.
2. Search the codebase (`Glob`, `Grep`) to locate the conflicting code.
3. Research the issue (web search, crate documentation, Slint and Rust source) to find the root cause and candidate solutions.
4. Propose the fix — pinned version, feature flag, cfg gate, or code change — with rationale. Wait for user approval before applying.
5. If the fix reveals a general pattern (e.g., how a crate must be gated per target), document it in the appropriate rules file (`rust-code-style.md`, `slint-code-style.md`, or `general-programming-practice.md`).

## Step 3 — Plan the fix `[task-manager]`

1. Write the confirmed (and optimized) fix description into `@.github/speckit.bug.[BugID].report.md`.
2. Break the fix into atomic tasks and subtasks per `task-planning.md`, labeling each with its responsible agent.
3. Present the plan to the user and get approval.
4. Commit with message `Solution plan [BugID]`.

## Step 4 — Delegate `[task-manager]`

Brief and invoke the implementing agent(s) following `implement-tasks/SKILL.md` Step 2: goal, files to modify/read (absolute paths), ordered steps, and any constraints the agent might not already know.

## Step 5 — Implement `[slint-developer / rust-developer]`

The agent implements the fix, verifies the build, and suggests a commit message per `commit-msg-format.md` — following its own non-negotiable rules and `implement-tasks/SKILL.md` Steps 3 and 4a. It stops after suggesting the commit message; it does not review, commit, or mark the bug done.

## Step 6 — Test `[slint-tester]`

Invoke **slint-tester** to add or update CRUD/unit tests covering the fixed behavior, following `testing-tasks/SKILL.md` Approach 1 and `slint-test-format.md`. Skip this step for bugs with no testable behavior change — e.g. UI layout, color, or appearance fixes.

## Step 7 — Close `[task-manager]`

1. Follow `implement-tasks/SKILL.md` Step 4b: present the commit message, get review approval, commit, and mark the bug `(done)` in `speckit.specify.prompt.md`.
2. Report the closed BugID and outcome to the user.

