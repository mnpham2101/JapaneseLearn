# Subtask 6.9.2 — Vocabulary CRUD handlers wired in init()

**Agent**: rust-developer  
**Parent task**: 6.9  
**Depends on**: 6.9.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, all lesson and word CRUD callbacks declared in `VocabularyAppLogic` are wired in `init()`. Creating a lesson appends it to the live model and saves; deleting removes it and resets the index; adding/editing/deleting words updates the selected lesson's word list and saves. `cargo build`, `cargo clippy` (zero warnings), and `cargo test` pass.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/src/lib.rs` | modify | register all CRUD callbacks in `init()` |

## Components / Modules

No new modules. All handlers are closures registered in `init()` on `VocabularyAppLogic`.

## Functions / Callbacks

Wire these in `init()` (in the same style as `lib/flashcard/src/lib.rs`):

- **`on_lesson_create_confirmed(name)`** — append `VocabularyLessonModel { name, words: [] }` to `VecModel<VocabularyLessonModel>`, call `save_vocabulary()`.
- **`on_lesson_delete_confirmed()`** — remove entry at `logic.get_selected_lesson_index()`, reset `selected-lesson-index = -1`, call `save_vocabulary()`.
- **`on_word_add_confirmed(lesson_idx, spelling, kanji, meaning, word_type)`** — append new `VocabularyWordModel` to `lesson_list[lesson_idx].words` VecModel, call `save_vocabulary()`.
- **`on_word_delete_confirmed(lesson_idx, word_idx)`** — remove word at index, call `save_vocabulary()`.
- **`on_word_field_changed(lesson_idx, word_idx, spelling, kanji, meaning, word_type)`** — replace word at index with updated fields, call `save_vocabulary()`.
- **`on_word_tense_add_confirmed(lesson_idx, word_idx, tense_name, conjugation)`** — append `TenseEntryModel` to word's tenses VecModel, call `save_vocabulary()`.
- **`on_word_tense_delete_confirmed(lesson_idx, word_idx, tense_idx)`** — remove tense at index, call `save_vocabulary()`.
- **`on_word_example_add_confirmed(lesson_idx, word_idx, example)`** — append example string to word's examples VecModel, call `save_vocabulary()`.
- **`on_word_example_delete_confirmed(lesson_idx, word_idx, example_idx)`** — remove example at index, call `save_vocabulary()`.

Each handler follows the weak-handle pattern:
```rust
let ui_weak = ui.as_weak();
logic.on_lesson_create_confirmed(move |name| {
    let ui = ui_weak.unwrap();
    let logic = ui.global::<VocabularyAppLogic>();
    // ...mutate VecModel...
    save_vocabulary(&slint_to_lessons(&logic));
});
```

## Patterns and Notes

- Follow the same mutation pattern as `on_stack_create_confirmed` in `lib/flashcard/src/lib.rs`: get the current `ModelRc`, downcast to `VecModel`, push, then call `logic.set_lesson_list(...)`.
- Nested model mutation (word inside lesson): get `lesson_list`, get `lesson.words` as `VecModel`, mutate words VecModel. The outer `lesson_list` does **not** need to be re-set if only the inner words VecModel is mutated in place — Slint propagates the change automatically.
- `kanji` and `word_type` arrive as `SharedString`; treat empty string as "absent" (store as-is, Rust shows it as empty — a `None` is not needed at the Slint layer).
- This commit clears all dead-code warnings introduced in 6.9.1 — `cargo clippy` must pass with zero warnings.
