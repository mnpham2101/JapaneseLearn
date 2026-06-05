# Subtask 6.8.1 — Word form: spelling, kanji, meaning, type fields

**Agent**: slint-developer  
**Parent task**: 6.8  
**Depends on**: 6.7.2  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `VocabularyAppLogic` declares word add/edit/delete callbacks, and a `WordForm` component in `lib/vocabulary/ui/components/word_form.slint` provides TextInput fields for spelling, kanji, meaning, and word type. The lesson detail view in `VocabularyPage` shows the word list for the selected lesson and an "Add Word" button that reveals the `WordForm`. The build passes; no Rust CRUD logic yet.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add word CRUD callbacks and `selected-word-index` property |
| `lib/vocabulary/ui/components/word_form.slint` | create | `WordForm` component with spelling, kanji, meaning, type fields |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | add lesson detail pane showing word list and `WordForm` |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | export `WordForm` |

## Components / Modules

- **`WordForm`** — form component with four `TextInput` fields: `spelling-input`, `kanji-input` (labelled "optional"), `meaning-input`, `type-input` (labelled "optional", e.g. "noun"). Confirm/Cancel `CommonBtn` buttons at the bottom. Fires callbacks when confirmed. File: `word_form.slint`.

- **`VocabularyPage`** (modified) — when `selected-lesson-index >= 0`, show a lesson detail pane alongside the lesson list. The detail pane lists the selected lesson's words (each row: spelling, kanji, meaning, type text, plus delete "✕" button). An "Add Word" button reveals `WordForm` using the vertically stacked pattern.

## Functions / Callbacks

Add to `VocabularyAppLogic`:

```slint
in-out property <int> selected-word-index: -1;

callback word-add-confirmed(lesson-idx: int, spelling: string, kanji: string, meaning: string, word-type: string);
callback word-delete-confirmed(lesson-idx: int, word-idx: int);
callback word-field-changed(lesson-idx: int, word-idx: int, spelling: string, kanji: string, meaning: string, word-type: string);
```

Wire in `VocabularyPage`:
- Word form confirmed → `VocabularyAppLogic.word-add-confirmed(selected-lesson-index, ...fields...)` then hide form
- Word row delete → `VocabularyAppLogic.word-delete-confirmed(selected-lesson-index, i)`
- Word row field edited → `VocabularyAppLogic.word-field-changed(selected-lesson-index, i, ...)`

## Patterns and Notes

- `kanji` and `word-type` are optional fields; pass empty string when blank — Rust will convert `""` to `None` where needed.
- Follow existing `flashcard_stack.slint` inline-edit pattern for the word row text fields.
- Import `@styles` tokens; no hardcoded colors.
- Verify `cargo build --bin japanese_learn` passes before committing.
