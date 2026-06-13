# Subtask 6.7.2 — Implement LessonList component

**Agent**: slint-developer  
**Parent task**: 6.7  
**Depends on**: 6.7.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `lib/vocabulary/ui/components/lesson_list.slint` contains a `LessonList` component that renders a scrollable list of lesson names and an inline create-lesson form. A `VocabularyPage` component in `lib/vocabulary/ui/pages/vocabulary_page.slint` hosts the `LessonList` and wires all callbacks to `VocabularyAppLogic`. The build passes; no Rust CRUD logic yet.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/lesson_list.slint` | create | `LessonList` component |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | create | `VocabularyPage` component hosting `LessonList` |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | export `VocabularyPage` and `LessonList` |

## Components / Modules

- **`LessonList`** — scrollable list of `VocabularyLessonModel` entries. Each row shows the lesson name (`CommonBtn` styled as a label, checkable for selection) and a delete `CommonBtn` ("✕"). Below the list: a "Create Lesson" `CommonBtn` that reveals an inline form (TextInput + Confirm/Cancel buttons) using the "Vertically stacked up components pattern" from `slint-code-style.md`. Fires `lesson-create-confirmed`, `lesson-delete-confirmed`, and updates `selected-lesson-index` on `VocabularyAppLogic`.

- **`VocabularyPage`** — top-level page component. Contains a `LessonList` bound to `VocabularyAppLogic.lesson-list`. File: `vocabulary_page.slint`; component name `VocabularyPage` follows the `*Page` suffix convention.

## Functions / Callbacks

Wire inside `VocabularyPage` (Slint-side only — no Rust handlers yet):
- Lesson row clicked → `VocabularyAppLogic.selected-lesson-index = i`
- Create form confirmed → `VocabularyAppLogic.lesson-create-confirmed(name-input.text)` then clear and hide form
- Delete button clicked → `VocabularyAppLogic.selected-lesson-index = i; VocabularyAppLogic.lesson-delete-confirmed()`

## Patterns and Notes

- Import and use `@styles` (`Tokens`) for all colors and sizes — no hardcoded values.
- Use `CommonBtn` from `@flashcard` library for buttons (import via the flashcard library path alias).
- Follow the "Vertically stacked up components pattern": use `if show-create-form: CreateLessonFormContainer { ... }` with `property <bool> show-create-form: false`.
- Each component in its own file; file name is kebab-case of the component name (`VocabularyPage` → `vocabulary_page.slint`).
- Verify `cargo build --bin japanese_learn` passes before committing.
