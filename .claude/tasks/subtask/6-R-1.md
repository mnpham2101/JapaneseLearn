# Subtask 6.R.1 — Move StudySessionView to lib/flashcard

**Agent**: slint-developer
**Parent task**: 6.R.1
**Depends on**: none
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: `ui/pages/study_session_view.slint` no longer exists at the root `ui/pages/` path. It lives at `lib/flashcard/ui/components/study_session_view.slint`. `flashcard_lib.slint` exports `StudySessionView`. `study_page.slint` imports `StudySessionView` from `@flashcard` instead of the local file. `cargo build --bin japanese_learn` passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `ui/pages/study_session_view.slint` | delete (move source) | File removed from root ui/pages/ |
| `lib/flashcard/ui/components/study_session_view.slint` | create | Exact content of `ui/pages/study_session_view.slint` — no logic changes |
| `lib/flashcard/ui/flashcard_lib.slint` | modify | Add `export { StudySessionView } from "components/study_session_view.slint";` |
| `ui/pages/study_page.slint` | modify | Replace `import { StudySessionView } from "study_session_view.slint";` with `StudySessionView` added to the existing `@flashcard` import block |

## Components / Modules

- **StudySessionView** — unchanged component; only its file location and import path change.

## Functions / Callbacks

None — this is a pure file move with import path update.

## Patterns and Notes

- The component content must be identical to the original — do not edit logic.
- In `study_page.slint`, `StudySessionView` must be added to the existing `import { ..., StudySessionView } from "@flashcard"` block; remove the separate local import line.
- Verify `cargo build --bin japanese_learn` passes before suggesting commit.
