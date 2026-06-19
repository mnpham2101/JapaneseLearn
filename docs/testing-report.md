# Testing & Performance Report — Phase 8

`cargo build --workspace`, `cargo clippy --workspace`, and `cargo test --workspace -- --test-threads=1` all pass clean as of this report.

## Test counts by crate

| Crate / test target | Tests | Kind |
|---|---|---|
| `lib/exercise_generator` (inline `#[cfg(test)]`) | 4 | unit |
| `lib/persistent_data` (`markdown_io.rs`) | 5 | unit |
| `lib/persistent_data` (doc comments) | 2 | doctest |
| `lib/vocabulary` (`vocabulary_markdown_io.rs`) | 8 | unit |
| `test/flashcard/tests/study_page.rs` | 11 | integration (`slint::testing`) |
| `test/vocabulary/tests/vocabulary_page.rs` | 7 | integration |
| `test/vocabulary/tests/matching_exercise_view.rs` | 4 | integration |
| `test/vocabulary/tests/generate_exercises_notification.rs` | 3 | integration |
| **Total** | **44** | |

`lib/flashcard`'s own crate has no inline unit tests — its CRUD/persistence behavior is covered entirely by `test/flashcard`'s integration suite (the project's established pattern per `.claude/rules/slint-test-format.md`).

## Phase 8 additions

- **8.1** — added `study_page_progress_no_selection` and `study_page_progress_empty_stack` to `test/flashcard/tests/study_page.rs`, closing the two edge cases in `update_progress()` that weren't previously exercised (no stack selected; a selected stack with zero cards).
- **8.2** — `on_known_changed` (`lib/flashcard/src/lib.rs`) now early-returns when the incoming `known` value already matches the card's current value, skipping the `flashcard_list` rebuild, `set_flashcard_list`, `save_stacks()` (a full JSON file write), and `update_progress()`. This matters because the Phase 8.B fix (`changed data => { known = data.known; }` in `Flashcard`) makes `known-changed` fire on every Prev/Next card navigation, not just on an actual toggle — without this guard, every navigation during a study session would trigger a disk write. Verified by `study_page_known_changed_repeated_noop`.

## Known pre-existing warnings (out of scope)

Two warnings appear in `cargo build`/`cargo clippy` output that predate all Phase 7/8/8.B/8.V work and are not addressed here, since they stem from the project's existing dual bin/cdylib structure (see `CLAUDE.md` § Build & Run, "parallel bin+cdylib builds share a PDB name") rather than anything introduced in this round of changes:

- `Cargo.toml: file src/main.rs found to be present in multiple build targets` — the root crate's `[lib]` target and the implicit `[[bin]]` target both point at `src/main.rs`.
- `function 'main' is never used` — surfaces only when the `[lib]` (cdylib) target is checked in isolation, since `main()` is the bin entry point, not part of the lib's public API.

A number of `Exported component '...' doesn't inherit Window. No code will be generated for it` notices also appear during `lib/vocabulary`'s build — these are expected `slint-build` informational output for components that are exported as reusable widgets (not top-level windows), not Rust warnings, and are not part of `cargo clippy`'s lint output.

## Rendering performance

No rendering bottleneck was identified or reported for Windows/WebAssembly at the app's current data scale (a handful of stacks/lessons, tens of cards each). The dominant existing pattern across every CRUD handler — `Vec<T> = model.iter().collect()`, mutate, `ModelRc::new(VecModel::from(...))` — reallocates the full list on every edit. This is a real inefficiency at large list sizes, but rewriting it to incremental `VecModel` mutation (e.g. `model.set_row_data`) is a broad cross-cutting refactor across every CRUD handler in `lib/flashcard` and `lib/vocabulary`, not a contained Phase 8 task, and was not undertaken speculatively per `.claude/rules/general-programming-practice.md` ("prioritize readability... don't add abstractions beyond what the task requires"). The one concrete, scoped optimization found and fixed is the Task 8.2 redundant-write guard above.
