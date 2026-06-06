# Subtask 6.R.5 — Create FlashcardManagerView

**Agent**: slint-developer
**Parent task**: 6.R.5
**Depends on**: 6.R.1
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: `lib/vocabulary/ui/components/flashcard_manager_view.slint` exists and exports `FlashcardManagerView`. The component is exported from `lib/vocabulary/ui/vocabulary_lib.slint`. `cargo build --bin japanese_learn` passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/flashcard_manager_view.slint` | create | New `FlashcardManagerView` component |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | Add `export { FlashcardManagerView } from "components/flashcard_manager_view.slint";` |

## Components / Modules

- **FlashcardManagerView** — self-contained component encapsulating the entire flashcard management UI that currently lives under the `if root.active-topic == 0` block in `ui/pages/study_page.slint`. It is placed in `lib/vocabulary` so that `VocabularyPage` can embed it without a circular dependency (vocabulary → flashcard is already an allowed dependency direction).

## Layout Structure (mirrors the Flashcard tab block in study_page.slint)

```
FlashcardManagerView (Rectangle, background: page-background)
  // Background dismiss TouchArea (lowest z-order)
  TouchArea
    clicked => { FlashcardAppLogic.selected-stack-index = -1; }

  VerticalLayout (padding: 20px, spacing: 20px)
    // Header row
    HorizontalLayout (spacing: 10px)
      Text "Study Mode" (horizontal-stretch: 1)
      CommonBtn "Import" (clicked => FlashcardAppLogic.import-stack-clicked())
      CommonBtn "Export" (clicked => FlashcardAppLogic.export-stack-clicked())

    // Vertically stacked pattern (alignment: end)
    VerticalLayout (alignment: end, vertical-stretch: 1)
      if FlashcardAppLogic.study-session-active: StudySessionView { }

      if !study-session-active && show-create-form: Rectangle (create-stack form, height: 140px)
        // Stack name TextInput + Confirm + Cancel

      if !study-session-active && show-stack: FlashcardStack
        // All callbacks wired to FlashcardAppLogic

      if !study-session-active && selected-stack-index == -1: HorizontalLayout
        // CommonList + FlashcardList centered at 400px
```

## Local Properties

- `property <bool> show-stack: FlashcardAppLogic.selected-stack-index >= 0` — derived, not stored.
- `property <bool> show-create-form: false` — local state.
- `changed show-stack => { if (show-stack) { show-create-form = false; } }` — reset form when stack opens.

## Functions / Callbacks

All callback wiring mirrors exactly what is in the `if root.active-topic == 0` block of `ui/pages/study_page.slint`. No new callbacks.

## Patterns and Notes

- Import `StudySessionView`, `FlashcardAppLogic`, `FlashcardStack`, `FlashcardList`, `CommonList`, `CommonBtn`, `FlashcardStackModel` all from `@flashcard`.
- Import `Tokens` from `@styles`.
- The create-stack form (TextInput + Confirm/Cancel) is identical to the one in `study_page.slint` — copy it verbatim, then it will be deleted from `study_page.slint` in task 6.R.7.
- `StudySessionView` is now in `@flashcard` (after task 6.R.1) — import it from there, not from a local path.
- This component is a complete extraction — after 6.R.5 the Flashcard tab block in `study_page.slint` remains until 6.R.7 removes it.
