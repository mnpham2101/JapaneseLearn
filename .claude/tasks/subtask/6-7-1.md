# Subtask 6.7.1 — Add lesson CRUD callbacks to VocabularyAppLogic

**Agent**: slint-developer  
**Parent task**: 6.7  
**Depends on**: 6.6  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `VocabularyAppLogic` in `lib/vocabulary/ui/vocabulary_app_logic.slint` declares the lesson list property, selected-lesson-index property, and lesson CRUD callbacks needed for the lesson list UI. No Rust handler is registered yet — the build passes with stub callbacks.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add `lesson-list`, `selected-lesson-index`, and lesson CRUD callbacks |

## Components / Modules

- **`VocabularyAppLogic`** — the library's global singleton. After this subtask it additionally exposes `lesson-list` (the mutable lesson list model) and callbacks for creating/deleting lessons.

## Functions / Callbacks

Add these declarations inside `VocabularyAppLogic`:

```slint
in-out property <[VocabularyLessonModel]> lesson-list;
in-out property <int> selected-lesson-index: -1;

callback lesson-create-confirmed(name: string);
callback lesson-delete-confirmed();
```

- `lesson-list` — the live model the `LessonList` component binds to.
- `selected-lesson-index` — set by the UI when a lesson entry is clicked; -1 means no selection.
- `lesson-create-confirmed(name)` — fired when the user confirms a new lesson name.
- `lesson-delete-confirmed()` — fired when the user confirms deletion; operates on `selected-lesson-index`.

## Patterns and Notes

- Follow the same property/callback declaration pattern as `FlashcardAppLogic` in `lib/flashcard/ui/flashcard_app_logic.slint`.
- `VocabularyLessonModel` and `VocabularyWordModel` are already defined in `vocabulary_app_logic.slint` from task 6.6 — do not redefine them.
- No Rust handler is needed yet (dead-code warning is acceptable per `atomic-commit-rule.md`).
- Verify `cargo build --bin japanese_learn` passes before committing.
