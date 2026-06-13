# Subtask 6.R.4 — Create LessonStackList

**Agent**: slint-developer
**Parent task**: 6.R.4
**Depends on**: 6.R.2
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: `lib/vocabulary/ui/components/lesson_stack_list.slint` exists and exports `LessonStackList`. The component is exported from `lib/vocabulary/ui/vocabulary_lib.slint`. `cargo build --bin japanese_learn` passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/lesson_stack_list.slint` | create | New `LessonStackList` component |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | Add `export { LessonStackList } from "components/lesson_stack_list.slint";` |

## Components / Modules

- **LessonStackList** — scrollable list of lessons rendered as `LessonStackLabel` rows, plus an inline create-lesson form and an "＋ Add Lesson" button. Follows the **Vertically stacked pattern** (`alignment: LayoutAlignment.end`).

## Layout Structure

```
LessonStackList (VerticalLayout, alignment: end)
  Flickable (vertical-stretch: 1)
    VerticalLayout (lesson rows)
      for lesson[i] in VocabularyAppLogic.lesson-list:
        HorizontalLayout
          LessonStackLabel (horizontal-stretch: 1, checked: selected-lesson-index == i, clicked => selected-lesson-index = i)
          CommonBtn (width: 36px, text: "✕", clicked => lesson-delete-confirmed)
  if show-create-form: Rectangle (create-lesson form, height: 64px)
    HorizontalLayout
      TextInput (lesson name)
      CommonBtn "Confirm" (primary, clicked => lesson-create-confirmed + hide form)
      CommonBtn "Cancel" (clicked => hide form)
  if !show-create-form: CommonBtn "＋ Add Lesson" (primary, height: 36px, clicked => show form)
```

## Functions / Callbacks

- Local `property <bool> show-create-form: false` — controls form visibility.
- On Confirm: invoke `VocabularyAppLogic.lesson-create-confirmed(name-input.text)`, clear input, set `show-create-form = false`.
- On delete button: invoke `VocabularyAppLogic.lesson-delete-confirmed()` after setting `VocabularyAppLogic.selected-lesson-index = i`.
- On `LessonStackLabel` clicked: set `VocabularyAppLogic.selected-lesson-index = i`.

## Patterns and Notes

- Import `LessonStackLabel` from `"lesson_stack_label.slint"`, `CommonBtn` from `@flashcard`, `VocabularyAppLogic` from `"../vocabulary_app_logic.slint"`, `Tokens` from `"@styles"`.
- The create-lesson form mirrors the pattern in `lib/vocabulary/ui/components/lesson_list.slint` exactly — same field styling, same token references.
- This component supersedes `LessonList` for the new vocabulary page but `LessonList` is not deleted (it may still be referenced elsewhere until 6.R.6 lands).
- Do not add selection-to-detail navigation here — the parent (`VocabularyPage`) observes `selected-lesson-index` and conditionally shows `LessonDetailView`.
