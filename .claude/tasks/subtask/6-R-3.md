# Subtask 6.R.3 — Create LessonDetailView

**Agent**: slint-developer
**Parent task**: 6.R.3
**Depends on**: none
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: `lib/vocabulary/ui/components/lesson_detail_view.slint` exists and exports `LessonDetailView`. The component is exported from `lib/vocabulary/ui/vocabulary_lib.slint`. `cargo build --bin japanese_learn` passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/lesson_detail_view.slint` | create | New `LessonDetailView` component |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | Add `export { LessonDetailView } from "components/lesson_detail_view.slint";` |

## Components / Modules

- **LessonDetailView** — full-page component that shows a selected lesson's content. Layout: a header row (lesson name `Text` + close `CommonBtn`) followed by `LessonDetailPane` below (takes remaining height). Exposes `callback close-clicked` that is emitted by the close button.

## Functions / Callbacks

- `callback close-clicked` — emitted when the user clicks the close `CommonBtn` in the header. The parent wires this to reset `VocabularyAppLogic.selected-lesson-index = -1`.

## Patterns and Notes

- Import `CommonBtn` from `@flashcard`, `LessonDetailPane` from `"lesson_detail_pane.slint"`, `VocabularyAppLogic` from `"../vocabulary_app_logic.slint"`, and `Tokens` from `"@styles"`.
- Header row: `Text` showing `VocabularyAppLogic.lesson-list[VocabularyAppLogic.selected-lesson-index].name` (guard the index with `>= 0` check or ternary), and a `CommonBtn` with text `"✕"` (width 36px) whose `clicked` emits `root.close-clicked()`.
- Below the header: `LessonDetailPane` with `vertical-stretch: 1`.
- The component is a `VerticalLayout` or `Rectangle` root — keep it simple.
- Do not add selection logic here — the parent controls which lesson index is active.
