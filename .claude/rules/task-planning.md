---
paths:
  - .claude/tasks/tasks.md
  - .claude/tasks/subtask/*.md
  - .claude/tasks/architecture/task-*.md
---

# Task Planning Rules

**Format reference — used by task-manager (task/subtask files) and project-owner (task-scoped architecture plan files).** Owns: ID scheme, writing locations, format templates, and commit ownership for all three artifact types below.  
For task-manager's execution procedure (phases, gates, agent dispatch, when it requests an architecture plan file): see `task-manager.md`.  
For project-owner's procedure on authoring an architecture plan file: see `project-owner.md` § "On a task-manager request to document a multi-library/module task".  
For commit workflow: see `implement-tasks/SKILL.md`.

---

## Work Priority Order

Tasks within a phase must be ordered and grouped according to this dependency chain:

```
1. UI development        → slint-developer (first — defines callback signatures)
2. Service library       → rust-developer  (parallel with UI if no Slint dependency)
3. Test writing          → slint-tester    (parallel with step 2, after step 1)
4. Library integration   → rust-developer  (after both libraries in steps 1–2 are done)
5. Frontend-backend wire → rust-developer  (last — registers init() callbacks)
```

**Rule by library type:**

| Library type | When to schedule | Assignee |
|---|---|---|
| libA UI scaffold + Slint components | First | slint-developer |
| libD / libB pure Rust service (no Slint) | Parallel with libA UI tasks | rust-developer |
| libA Rust CRUD handlers | After UI callbacks are defined | rust-developer |
| Integration: libA calling libD/libB | After both libraries build | rust-developer |
| `init()` wiring in caller lib | Last in each feature chain | rust-developer |

**Parallel group notation** — mark every parallel group with a blockquote before the first task in the group:

```
> Tasks 6.2.1 and 6.3 are independent — may run in parallel.

> Tasks 6.4.1 and 6.4.2 both modify `vocabulary_app_logic.slint` — implement sequentially.
```

---

## Task ID Format

| Level | Format | Example | Location |
|---|---|---|---|
| Task | `M.N` | `6.2` | `.claude/tasks/tasks.md` |
| Subtask | `M.N.X` | `6.2.1` | `.claude/tasks/tasks.md` (reference) + `.claude/tasks/subtask/M-N-X.md` (detail) |

- `M` = phase number (matches the phase in `.claude/specs/plan.md`)
- `N` = sequential task number within the phase
- `X` = sequential subtask number within task `M.N`

---

## Writing Locations

| What | File | Authored by |
|---|---|---|
| All tasks (`M.N`) and subtask references (`M.N.X`) | `.claude/tasks/tasks.md` | task-manager |
| Subtask detail (goals + technical approach) | `.claude/tasks/subtask/M-N-X.md` (one file per subtask) | task-manager |
| Task-scoped architecture plan (only when a task spans ≥2 libraries/modules) | `.claude/tasks/architecture/task-M-N.md` (e.g. `task-6-9.md` for Task 6.9) | project-owner (on task-manager's request) |

**Never** write subtask implementation detail inline in `.claude/tasks/tasks.md` — keep the tasks file scannable. A subtask entry in the tasks file is one line plus a file reference.

**Commit ownership**: each author commits what it writes, in its own atomic commit, separate from any implementation commit — task-manager commits the task/subtask files before delegating to developer agents; project-owner commits the architecture plan file it authors. Neither bundles the other's planning docs into its commit.

---

## Task Format in `.claude/tasks/tasks.md`

### Simple task (single deliverable, no subtasks needed)

```
- [ ] 6.2 **[rust-developer]** Implement `FlashcardExerciseTransformer` in `lib/exercise_generator`: one card per spelling word; two cards (kanji + spelling) when kanji is present. Unit tests inline. **Depends on 6.1.**
```

File reference format for subtasks (link text = ID only, path = `.claude/tasks/subtask/`):
```
- [ ] 6.4.1 Lesson list + create/delete callbacks — see [6-4-1.md](.claude/tasks/subtask/6-4-1.md)
```

Rules:
- One line: checkbox + task ID + agent label + concise goal + dependency declaration.
- No multi-sentence prose. If the description exceeds one clear sentence, use subtasks.

### Task with subtasks

```
- [ ] 6.4 **[slint-developer]** Vocabulary lesson CRUD UI in `lib/vocabulary/ui/`: lesson list, word form with all fields, VocabularyAppLogic global.
  - [ ] 6.4.1 Lesson list + create/delete callbacks — see [6-4-1.md](.claude/tasks/subtask/6-4-1.md)
  - [ ] 6.4.2 Word form: spelling, kanji, meaning, type fields — see [6-4-2.md](.claude/tasks/subtask/6-4-2.md)
  - [ ] 6.4.3 Word form: tense list and example list — see [6-4-3.md](.claude/tasks/subtask/6-4-3.md)
```

Rules:
- The parent task (`6.4`) states the overall goal in one line.
- Each subtask is one indented line with its ID and a file reference. No description beyond the file link.
- The detail lives entirely in the subtask file.

### Subtask reference format

```
- [ ] 6.4.1 Lesson list + create/delete callbacks — see [6-4-1.md](.claude/tasks/subtask/6-4-1.md)
```

### Dependency declaration

Every task and subtask that has a non-trivial predecessor ends with `**Depends on M.N.**` or `**Depends on M.N.X.**`.

### Test task format

```
- [ ] 6.T **[slint-tester]** Test vocabulary CRUD on VocabularyPage.
  - Callbacks to invoke: `invoke_lesson_create_confirmed(name)`, `invoke_word_add_confirmed(lesson_idx, spelling, kanji, meaning)`
  - Properties to assert: `get_lesson_list().row_count()`, `get_lesson_list().row_data(0).words.row_count()`
  - Behaviors: creating a lesson appends it; adding a word appends it to the selected lesson.
  - Covers: Task 6.4 + 6.5
  **Depends on 6.5 (rust-developer).**
```

---

## Subtask File Format

File name: `M-N-X.md`  
Location: `.claude/tasks/subtask/`

```markdown
# Subtask M.N.X — [one-line title]

**Agent**: [slint-developer | rust-developer | slint-tester]  
**Parent task**: M.N  
**Depends on**: M.N.(X-1) *(or "none" if first in chain)*  
**Commit**: one commit, one logical change

---

# Subtask Goals

One paragraph: what must be true after this subtask is committed. State the observable outcome (build passes, callback declared, test passes), not the steps taken.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add `callback lesson-create-confirmed(name: string)` |
| `lib/vocabulary/ui/components/lesson_list.slint` | create | new `LessonList` component |

## Components / Modules

List every Slint component or Rust module involved. For each:
- **Name** — purpose in one sentence.
- If new: state the minimal public interface (exported properties, callbacks, or public functions).
- If modified: state only what changes, not what stays the same.

## Functions / Callbacks

List individual functions or callbacks that are added or changed in this subtask. Keep to the single logical change — entire modules cannot be added in one subtask.

Example:
- `on_lesson_create_confirmed` closure in `lib/vocabulary/src/lib.rs`: reads `name` from callback arg, creates `VocabularyLessonModel { name, words: [] }`, appends to `VecModel`, pushes back.

## Patterns and Notes

Optional. Include only if there is a non-obvious design decision, an existing pattern to follow, or a constraint the implementing agent must know:
- Reference the relevant section of `slint-code-style.md` or `rust-code-style.md` if a known pattern applies.
- State any invariant that must hold after this commit (e.g., "lesson list index must not change after creation").
- Flag any dead-code warning that is expected and acceptable per `atomic-commit-rule.md`.
```

---

## Task-Scoped Architecture Plan File Format

Required **only** when a task's subtasks change code across **≥2 libraries/modules** (e.g., a `lib/vocabulary` change that also calls into `lib/exercise_generator`). task-manager detects this during Phase 2 (Investigate) and invokes **project-owner** to author the file — task-manager does not draft this diagram itself.

File name: `task-M-N.md` (e.g. `task-6-9.md` for Task 6.9)
Location: `.claude/tasks/architecture/`

```markdown
# Task M.N — Architecture Plan
<!-- File: .claude/tasks/architecture/task-M-N.md -->

**Modules involved**: `lib/vocabulary`, `lib/exercise_generator`
**What changes between them**: [one sentence — the specific interaction this task adds or modifies]

---

## Modules & Roles (for this task only)

- `lib/vocabulary` (libA) — owns `VocabularyLesson` data and CRUD UI; in this task, it requests exercise generation from the transformer.
- `lib/exercise_generator` (libD) — pure transformer; in this task, it gains the entry point that converts a `VocabularyLesson` into `FlashcardStackData`.

## Interaction Diagram

\`\`\`plantuml
@startuml
actor User
participant "VocabularyAppLogic\n(lib/vocabulary)" as Vocab
participant "ExerciseGeneratorService\n(lib/exercise_generator)" as Gen

User -> Vocab : generate-exercise-clicked(lesson_idx)
Vocab -> Gen : generate(ExerciseRequest::Flashcard(lesson))
Gen --> Vocab : ExerciseOutput::FlashcardStackData
Vocab -> User : push stack to flashcard-list
@enduml
\`\`\`

## Notes
- [Any invariant or constraint the implementing agents must preserve across this boundary.]
```

Rules:
- Show **only** the modules, roles, and interaction(s) **this task** changes — never the whole-application component diagram (that's `architecture_diagram.puml`) and never the full set of inter-module interactions, only the one(s) this task adds or modifies.
- Keep the diagram to the actors and calls relevant to this task's data/control flow — not full class or sequence detail.
- Authorship and commit ownership: see § Writing Locations above.

---

## Format Rules

1. **Tasks are scannable; subtasks are actionable.** `tasks/tasks.md` is the map; subtask files in `tasks/subtask/` are the briefs.
2. **No implementation detail in `tasks/tasks.md`.** Keep entries to: checkbox, ID, agent label, one-line goal, dependency, subtask file reference.
3. **One subtask = one logical change = one commit.** Enforced by `atomic-commit-rule.md`.
4. **Architecture plan files are task-scoped, not project-scoped.** See § Task-Scoped Architecture Plan File Format above for the full spec and scope rules.
