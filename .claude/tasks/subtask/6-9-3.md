# Subtask 6.9.3 — Vocabulary markdown import/export

**Agent**: rust-developer  
**Parent task**: 6.9  
**Depends on**: 6.9.2  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `lib/vocabulary/src/vocabulary_markdown_io.rs` can parse and serialize the vocabulary markdown format. `VocabularyAppLogic` has `import-vocabulary-clicked` and `export-vocabulary-clicked` callbacks. The `init()` function wires both with rfd file dialogs. `cargo build` and `cargo test -p vocabulary` (inline unit tests) pass. The vocabulary markdown format is completely separate from the flashcard markdown format in `lib/persistent_data`.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/Cargo.toml` | modify | add `pulldown-cmark = "0.12"`, `rfd = { version = "0.15" }` |
| `lib/vocabulary/src/vocabulary_markdown_io.rs` | create | `parse_vocabulary` + `serialize_vocabulary` + unit tests |
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add `import-vocabulary-clicked`, `export-vocabulary-clicked` callbacks |
| `lib/vocabulary/src/lib.rs` | modify | expose module; wire import/export callbacks in `init()` |

## Components / Modules

- **`vocabulary_markdown_io`** — pure Rust module (no Slint types). Exposes:
  - `parse_vocabulary(source: &str) -> Vec<LessonData>` — parses the vocabulary markdown format using `pulldown-cmark` with `Options::ENABLE_TABLES`.
  - `serialize_vocabulary(lessons: &[LessonData]) -> String` — serializes lessons back to the vocabulary markdown format.
  - Inline `#[cfg(test)]` tests: single lesson with one word (no kanji), one word with kanji, tenses and examples, two lessons, empty input.

## Vocabulary Markdown Format

```
## Lesson Name

### spelling
kanji: 犬
meaning: dog
type: noun
tense: past|食べました
tense: negative|食べません
example: 犬が走る。
example: その犬は大きい。
```

Parsing rules:
- `## heading` → new `LessonData { name: heading, words: [] }`
- `### heading` → new word entry; `spelling = heading`
- `kanji: value` → `word.kanji = value` (ignore if blank)
- `meaning: value` → `word.meaning = value`
- `type: value` → `word.word_type = value`
- `tense: name|conjugation` → append `TenseData { name, conjugation }` (split on first `|`)
- `example: text` → append to `word.examples`

## Functions / Callbacks

Add to `VocabularyAppLogic`:

```slint
callback import-vocabulary-clicked();
callback export-vocabulary-clicked();
```

Wire in `init()`:

```rust
logic.on_import_vocabulary_clicked({
    let ui_weak = ui.as_weak();
    move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new().add_filter("Markdown", &["md"]).pick_file() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let lessons = vocabulary_markdown_io::parse_vocabulary(&content);
                    // convert to Slint models and set on VocabularyAppLogic
                    // also call save_vocabulary() to persist
                }
            }
        }
    }
});
// similar for on_export_vocabulary_clicked
```

## Patterns and Notes

- `parse_vocabulary` and `serialize_vocabulary` operate on `LessonData` (the same shadow structs from 6.9.1) — never on Slint types.
- All `std::fs` and `rfd` calls in the `init()` handlers must be inside `#[cfg(not(target_arch = "wasm32"))]` blocks; provide empty closures under WASM.
- Add "Import" and "Export" `CommonBtn` buttons to `VocabularyPage` (in `vocabulary_page.slint`) wired to `VocabularyAppLogic.import-vocabulary-clicked()` and `VocabularyAppLogic.export-vocabulary-clicked()`. This is the only Slint change permitted in this commit.
- Unit tests go inline in `vocabulary_markdown_io.rs` using `#[cfg(test)]` — follow the same pattern as `markdown_io.rs` in `lib/persistent_data`.
