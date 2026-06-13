# Subtask 6.D.2 — Auto-load default vocabulary on first launch

**Agent**: rust-developer  
**Parent task**: 6.D  
**Depends on**: 6.D.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, when the application starts and `vocabulary.json` does not yet exist, `init()` calls `load_and_save_defaults()` which embeds the three JSON files via `include_str!()`, parses them into `Vec<LessonData>`, saves to `vocabulary.json`, and pushes the data into `VocabularyAppLogic`. If `vocabulary.json` already exists, the auto-load is skipped. All fs calls are gated `#[cfg(not(target_arch = "wasm32"))]`.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/src/lib.rs` | modify | add `load_and_save_defaults()` function; call it at start of `init()` when `vocabulary.json` is absent |

## Functions / Callbacks

- `load_and_save_defaults()` in `lib/vocabulary/src/lib.rs`:
  - Uses `include_str!("../ui/data/n5_verbs.json")`, `include_str!("../ui/data/n5_adjectives.json")`, `include_str!("../ui/data/n5_vocabulary.json")` to embed the three files at compile time.
  - Parses each with `serde_json::from_str::<Vec<LessonData>>(&json).unwrap_or_default()`.
  - Concatenates the three `Vec<LessonData>` into one combined list.
  - Calls `save_vocabulary(&combined)` to write `vocabulary.json`.
  - Returns `combined` so the caller can push it to the logic.
- In `init()`: immediately after `let logic = ui.global::<VocabularyAppLogic>();`, add:
  ```rust
  #[cfg(not(target_arch = "wasm32"))]
  {
      if !std::path::Path::new(VOCABULARY_FILE).exists() {
          let defaults = load_and_save_defaults();
          logic.set_lesson_list(lessons_to_slint(&defaults));
      } else {
          let saved = load_vocabulary();
          if !saved.is_empty() {
              logic.set_lesson_list(lessons_to_slint(&saved));
          }
      }
  }
  ```
  Remove the existing unconditional `load_vocabulary()` call that was already there (replace it with the branching logic above).

## Patterns and Notes

- `include_str!()` path is relative to the source file (`lib/vocabulary/src/lib.rs`), so `"../ui/data/n5_verbs.json"` resolves to `lib/vocabulary/ui/data/n5_verbs.json`.
- The function is `#[cfg(not(target_arch = "wasm32"))]` because `save_vocabulary` already has that gate.
- Dead-code warning is not expected — the function is called immediately in `init()`.
- The existing `load_vocabulary()` block at startup is replaced by the branching block above; do not leave duplicate loads.
