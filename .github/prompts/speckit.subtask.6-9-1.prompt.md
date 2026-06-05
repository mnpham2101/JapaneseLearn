# Subtask 6.9.1 — Vocabulary persistence: load/save vocabulary.json

**Agent**: rust-developer  
**Parent task**: 6.9  
**Depends on**: 6.8.2  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `lib/vocabulary/src/lib.rs` can serialize the vocabulary lesson list to `vocabulary.json` and restore it on startup. Shadow Rust structs (`LessonData`, `WordData`, `TenseData`) with `serde` derive macros are defined. `load_vocabulary()` is called at the start of `init()` to populate `VocabularyAppLogic`. All `std::fs` calls are gated with `#[cfg(not(target_arch = "wasm32"))]`. `cargo build` passes; CRUD handlers are not yet wired (dead-code warning is acceptable).

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/Cargo.toml` | modify | add `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"` |
| `lib/vocabulary/src/lib.rs` | modify | add shadow structs, `load_vocabulary()`, `save_vocabulary()`, call `load_vocabulary()` in `init()` |

## Components / Modules

- **`LessonData`** / **`WordData`** / **`TenseData`** — shadow Rust structs with `#[derive(Serialize, Deserialize, Clone, Default)]`. `WordData.kanji` and `WordData.word_type` are `String` (empty string = absent); `WordData.tenses` is `Vec<TenseData>`; `WordData.examples` is `Vec<String>`.

## Functions / Callbacks

- `fn load_vocabulary() -> Vec<LessonData>` — reads `vocabulary.json`; returns `vec![]` if file absent or parse error. Gated `#[cfg(not(target_arch = "wasm32"))]`; returns `vec![]` on WASM.
- `fn save_vocabulary(lessons: &[LessonData])` — serializes to `vocabulary.json`. Gated the same way.
- `fn lessons_to_slint(lessons: &[LessonData], ...) -> ModelRc<VocabularyLessonModel>` — converts `Vec<LessonData>` to Slint `ModelRc`. Called in `init()` after `load_vocabulary()`.

Call at the top of `init()`:
```rust
let saved = load_vocabulary();
if !saved.is_empty() {
    logic.set_lesson_list(lessons_to_slint(&saved).into());
}
```

## Patterns and Notes

- Follow the exact same pattern as `load_stacks()` / `save_stacks()` in `lib/flashcard/src/lib.rs`.
- File path is `vocabulary.json` in the working directory (same convention as `stacks.json`).
- `save_vocabulary` will be called by CRUD handlers in subtask 6.9.2 — it is defined here but called there (dead-code warning on `save_vocabulary` is acceptable in this commit).
- Do not add `serde`/`serde_json` to workspace `[workspace.dependencies]` unless already present — add them directly to `lib/vocabulary/Cargo.toml`.
