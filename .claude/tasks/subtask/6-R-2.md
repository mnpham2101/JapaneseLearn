# Subtask 6.R.2 — Create LessonStackLabel

**Agent**: slint-developer
**Parent task**: 6.R.2
**Depends on**: none
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: `lib/vocabulary/ui/components/lesson_stack_label.slint` exists and exports `LessonStackLabel`. The component is exported from `lib/vocabulary/ui/vocabulary_lib.slint`. `cargo build --bin japanese_learn` passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/lesson_stack_label.slint` | create | New `LessonStackLabel` component |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | Add `export { LessonStackLabel } from "components/lesson_stack_label.slint";` |

## Components / Modules

- **LessonStackLabel** — a clickable label representing one vocabulary lesson. Inherits `CommonBtn` (imported from `@flashcard`). Accepts `in property <VocabularyLessonModel> lesson` with a default of `{ name: "", words: [] }`. Sets `text: lesson.name`. Pattern mirrors `FlashcardLabel` exactly.

## Functions / Callbacks

None — pure display component; selection and deletion are wired by the parent (`LessonStackList`).

## Patterns and Notes

- Mirror `lib/flashcard/ui/components/flashcard_label.slint` as the reference pattern.
- Import `CommonBtn` from `@flashcard` and `VocabularyLessonModel` from `../vocabulary_app_logic.slint`.
- The `checked` property and click handler are set by the parent — do not add selection logic here.
