# Bug description

* The `stack.json`, storing data for `Flashcard` library should stay in `@lib/Flashcard/ui/data`. 
* The `Vocabulary.json`, storing data for `Vocabulary` should stay in `@lib/Vocabulary/ui/data`.
* This is to comply with the MVC architecture defined in `@.claude/rules/architecture.md`. If the architecture is not clear, fix it.
* At this end of this bug fix, provide the report with callflow indicating how the `Vocabulary.json` and `stack.json` files are read. Just give high level design flow, that includes major functions that open the file, read and parse the file, and the main module/library involves.

# Root cause analysis:

`stacks.json` (flashcard persistence) and `vocabulary.json` (vocabulary
persistence) are declared as bare relative-path constants:

```rust
// lib/flashcard/src/lib.rs
const STACKS_FILE: &str = "stacks.json";
// lib/vocabulary/src/lib.rs
const VOCABULARY_FILE: &str = "vocabulary.json";
```

`std::fs::write`/`read_to_string` resolve bare paths against the process's
**current working directory** — the project root when run via `cargo run` —
so both files land at the repo root, outside any library's folder structure.
That's the architecture violation the bug reports.

The bug's suggested fix — move them into `ui/data/` — would create a new
problem: `architecture.md` already defines `ui/data/` as "bundled **read-only**
seed data (markdown + JSON); embedded at compile time via `include_str!()`;
**never written at runtime**" (e.g. `lib/vocabulary/ui/data/n5_verbs.json`,
loaded into the running app at startup, then freely edited by the user — the
*bundled asset* is immutable, but its loaded copy is regular mutable app
state). `stacks.json`/`vocabulary.json` are the opposite: they **are** written
at runtime by `save_stacks`/`save_vocabulary` on every mutation. Placing them
in `ui/data/` would contradict that folder's documented contract — this is
the "architecture is not clear" the bug asks to resolve: the rules define
where compile-time seed data lives, but never define where **persisted
runtime state** belongs.

# Solution:

## 1. Clarify `architecture.md`
Sharpen the `ui/data/` description and add a new top-level `data/` folder
(sibling to `src/`, `ui/`, `lib/`, `test/`) so the two concerns are
unambiguous:

- `ui/data/` (inside each owning library) — compile-time bundled seed/default
  data (e.g. `n5_verbs.json`), embedded via `include_str!()`. Loaded into the
  running app at startup (and via "Restore Defaults") and from then on freely
  editable through the UI (add/edit/delete) — the bundled asset itself never
  changes; its loaded representation is ordinary mutable app state.
- `data/` (project root) — persisted runtime state: JSON serialization of the
  current app state, written by `save_*` / read by `load_*` at runtime (e.g.
  `data/stacks.json`, `data/vocabulary.json`). Git-ignored; the directory and
  files are created on first save. Kept at the project root rather than
  inside a library's source tree (`lib/<name>/...`) so runtime-mutable state
  stays clearly separate from source/build-time assets — mixing the two would
  be the same class of problem the bug reports, just relocated rather than
  resolved (stale save files showing up when browsing library source,
  packaging headaches, etc.).

## 2. Relocate the persisted files to the project-root `data/` folder
- `lib/flashcard/src/lib.rs`: `STACKS_FILE` → `"data/stacks.json"`
- `lib/vocabulary/src/lib.rs`: `VOCABULARY_FILE` → `"data/vocabulary.json"`
- Add `std::fs::create_dir_all("data")` to `save_stacks`/`save_vocabulary` so
  the directory exists on first run in a fresh checkout (it won't be present
  until the first save).

## 3. Update `.gitignore`
Replace the stale guesses (`stacks.json`, `lib/flashcard/stacks.json`,
`dist/stacks.json`, `test/flashcard/stacks.json`) with a single `data/` entry
covering both persisted files.

## 4. Update test cleanup paths
`test/flashcard/tests/study_page.rs`, `test/vocabulary/tests/vocabulary_page.rs`,
and `test/vocabulary/tests/generate_exercises_notification.rs` call
`std::fs::remove_file("stacks.json")` / `("vocabulary.json")` after each
persistence test — these must target `"data/stacks.json"` /
`"data/vocabulary.json"`, otherwise tests will leak artifacts into the new
`data/` folder.

## 5. Migrate existing local save files
Move the current root-level `stacks.json`/`vocabulary.json` (untracked local
dev data) into `data/` so no local progress is lost.

## 6. Document the call flow
Produce `speckit.bug.6.8.callflow.puml` — a high-level PlantUML sequence
diagram naming the major functions/modules that open, read/parse, and
persist `vocabulary.json` and `stacks.json` (`vocabulary::init` →
`load_vocabulary`/`save_vocabulary`, `flashcard::init` → `load_stacks`/
`save_stacks`/`save_current_list`), following the Bug 6.4 / 6.7 convention.