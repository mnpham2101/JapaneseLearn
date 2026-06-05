# Subtask 6.8.2 — Word form: tense list and example list

**Agent**: slint-developer  
**Parent task**: 6.8  
**Depends on**: 6.8.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `VocabularyAppLogic` declares tense and example CRUD callbacks, and `WordForm` has been extended with a tense list section (tense name + conjugation pairs, add/delete rows) and an example list section (plain text rows, add/delete). The build passes; no Rust CRUD logic yet.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add tense and example CRUD callbacks |
| `lib/vocabulary/ui/components/word_form.slint` | modify | add tense list and example list sections |

## Components / Modules

- **`WordForm`** (extended) — below the spelling/kanji/meaning/type fields, add two sections:
  1. **Tense list**: a `for tense[i]` loop showing each tense as two inline TextInputs (`tense-name`, `conjugation`) and a delete "✕" `CommonBtn`. Below the list: an "Add Tense" `CommonBtn` that reveals two TextInputs + Confirm/Cancel.
  2. **Example list**: a `for example[i]` loop showing each example as one TextInput and a delete "✕". Below: an "Add Example" `CommonBtn` with TextInput + Confirm/Cancel.

  The two sections are inside the same `WordForm` component — no new files needed.

## Functions / Callbacks

Add to `VocabularyAppLogic`:

```slint
callback word-tense-add-confirmed(lesson-idx: int, word-idx: int, tense-name: string, conjugation: string);
callback word-tense-delete-confirmed(lesson-idx: int, word-idx: int, tense-idx: int);
callback word-example-add-confirmed(lesson-idx: int, word-idx: int, example: string);
callback word-example-delete-confirmed(lesson-idx: int, word-idx: int, example-idx: int);
```

Wire in `WordForm` / `VocabularyPage`:
- Tense confirm → `VocabularyAppLogic.word-tense-add-confirmed(lesson-idx, word-idx, name, conjugation)` then hide inline form
- Tense delete → `VocabularyAppLogic.word-tense-delete-confirmed(lesson-idx, word-idx, i)`
- Example confirm → `VocabularyAppLogic.word-example-add-confirmed(lesson-idx, word-idx, example)` then hide inline form
- Example delete → `VocabularyAppLogic.word-example-delete-confirmed(lesson-idx, word-idx, i)`

## Patterns and Notes

- `lesson-idx` and `word-idx` must be passed as properties into `WordForm` from the caller (`VocabularyPage`).
- Use `VocabularyWordModel.tenses` (a `[TenseEntryModel]`) and `VocabularyWordModel.examples` (a `[string]`) for the `for` loops — ensure these fields are present on `VocabularyWordModel` from the 6.6 scaffold; add them if missing.
- Keep add-tense and add-example inline forms as local boolean properties on `WordForm` (one property per section), toggled by the respective "Add" button.
- Verify `cargo build --bin japanese_learn` passes before committing.
