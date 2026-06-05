---
agent: speckit.tasks
---
/speckit.tasks

# Task Execution Rules:
- Tasks are derived from the plan and requirements defined in `speckit.plan` and `speckit.specify`, and must align with the best practices outlined in `speckit.constitution`.
- Each task must have a clear scope, defined objectives, and be actionable with a specific deliverable.
- Tasks should be organized into phases that correspond to the development workflow defined in the constitution, ensuring incremental progress and maintainability.
- Tasks must be atomic and focused on a single objective to facilitate clear implementation and testing.
- Each task must be implemented, successfully built. and tested before moving on to the next task, following the incremental quality principle, and prompted for commit after each successful implementation.
- After successfull implementation of each task, review must be done on the commit codes and approved before moving on to the next task.
- Prefer reusing existing components and APIs over introducing new ones; do not add properties, functions, or components unless strictly required by the task scope.

# Task Breakdown for JapaneseLearn Application

## Phase 1: Foundation
- [x] 1.1 Setup Slint and Rust framework. Initialize Rust project structure with Cargo.
- [x] 1.1.1 Fix warning in build after task 1.1 implementation.
- [x] 1.2 Configure build targets for Windows and WebAssembly. Ensure `wasm-pack` and `wasm-opt` are integrated for WebAssembly builds.
- [x] 1.3.1 Create base UI layout (main window, page, navigation between pages). The pages are intended for study mode and review mode. Navigation between the pages is supported by a button.
- [x] 1.3.2 Implement Common UI components (buttons, lists) in Slint. The list provides placeholders for flashcard stacks and flashcards, but does not yet implement dynamic data binding or interactivity. The list provides buttons to add new flashcards and stacks, but the functionality is not yet implemented.
- [x] 1.3.3 Implement flashcard in Slint. The flashcard components should accept data models for the Japanese word and Vietnamese meaning. It must have front and back properties, and a mechanism to toggle between them. The flash card must have properties for user to mark the card as known or unknown. The flashcard should be designed to be reusable across different parts of the application (e.g., stack view, study mode).
- [x] 1.3.4 Improve Flashcard UI: move the known toggle button inside the card body at the top-right corner as a compact icon button showing ✓ (known) or ✗ (unknown). Replace the imperative `clicked` callback and `known-toggled` callback with a declarative two-way binding (`checkable: true; checked <=> known`). Document the prefer-binding-over-callback principle in code-style.md and speckit.clarify.prompt.md.
- [x] 1.4.1 Implement sample struct `FlashcardStackModel` and data models with hardcoded flashcard stacks. The `FlashcardStackModel` should have name of the stacks, and an array of flashcard data models. The flashcard data model should have properties for the Japanese word, Vietnamese meaning, and known/unknown status and should be imported from step 1.3.4. Implement `FlashcardList` that inherits CommonList. The `FlashcardList` component should accept an array of `FlashcardStackModel` instances and display Rectangle whose text is the name of the stack. Consider the `FlashcardList` as a list of flashcard stacks, and the Rectangle as a flashcard stack label. Refactore CommonList to be reusable for both flashcard stack list and flashcard list in stack view if possible. Modify the study-page to display the `FlashcardList` component with the hardcoded flashcard stack data instead of the placeholder list implemented in step 1.3.2.
- [x] 1.4.2 Implement `FlashcardLabel` in Slint that inherits from the common button component created in 1.3.2. The `FlashcardLabel` component should accept a flashcard stack data model created in 1.4.1. Modify Study-Page to allow displaying the flashcard stack on the same page when a `FlashcardLabel` is clicked. Use the pattern "Vertically stacked up components pattern" defined in speckit.clarify.prompt.md for the implementation.
- [x] 1.4.3 Modify `FlashcardStack` in `ui/components/flashcard_stack.slint` to display 2 plain text boxes per card (`jap-obj` and `explanation` as two `Text` elements, no flip or toggle behaviour). In `ui/pages/study_page.slint`, replace the placeholder `Rectangle` detail pane with the actual `FlashcardStack` component bound to `flashcardList[selected-stack-index]`; wire `close-clicked` to reset `selected-stack-index = -1`. The existing 3 hardcoded stacks are sufficient to verify multi-stack behaviour — task 1.4.4 is folded into this task.
  > Tasks 1.5 through 1.7 form a strict sequential chain — all touch overlapping `.slint` files and no parallel execution is available.

- [x] 1.5 **[slint-developer]** In `ui/components/flashcard_stack.slint`, give each card's two inline `Rectangle` border wrappers a local name following the `*Container` convention (e.g., `jap-field-container` and `meaning-field-container`). Create `ui/styles/tokens.slint` and define style tokens for at minimum: `border-color`, `border-width`, `border-radius`, and `card-field-background`. Replace all hardcoded style values in `flashcard_stack.slint` with these tokens. Import `tokens.slint` wherever it is used. Avoid hardcoded sizes — use relative or stretch layout instead.
- [x] 1.6.1 **[slint-developer]** Rename all Slint components across every `.slint` file to follow the PascalCase functional-suffix convention defined in `slint-code-style.md`: buttons → `*Btn`, text inputs → `*TxtBox`, layout containers and named `Rectangle` elements → `*Container`, pages → `*Page`. This includes `CommonButton` → `CommonBtn`, `NavButton` → `NavBtn`, and the `*Container` names introduced in task 1.5. Update every import statement and usage site across all `.slint` files. Verify `cargo build` passes after the rename. **Depends on 1.5.**
- [x] 1.6.2 **[slint-developer]** Move the `FlashcardModel` and `FlashcardStackModel` struct definitions out of `ui/components/flashcard.slint` into a new file `ui/model/flashcard_model.slint`. Update every `.slint` file that imports these structs to import from the new path. `src/main.rs` does not require changes — `slint::include_modules!()` resolves types transitively. Verify `cargo build` passes after the move. **Depends on 1.6.1.**
- [x] 1.7.1 **[slint-developer]** Create the `lib/flashcard/` library scaffold following the **libA** pattern from the constitution. Move the flashcard-specific `.slint` files (`flashcard.slint`, `flashcard_label.slint`, `flashcard_stack.slint`, `flashcard_list.slint`) and the model file from task 1.6.2 into `lib/flashcard/ui/`. Create `lib/flashcard/ui/main_lib.slint` that re-exports all flashcard components and models. Update imports in the main `ui/` files (`study_page.slint` etc.) to reference `@flashcard/` paths. Create `lib/flashcard/build.rs` (compile `ui/main_lib.slint` via `slint_build`), `lib/flashcard/Cargo.toml` (`name = "flashcard"`, `slint` dep, `slint-build` build-dep), and `lib/flashcard/src/lib.rs` with `slint::include_modules!()` and an empty `pub fn init()` stub. Convert the root `Cargo.toml` to a workspace with `members = [".", "lib/flashcard"]` and add `flashcard = { path = "lib/flashcard" }` to the root `[dependencies]`. Verify `cargo build` passes. **Depends on 1.6.2.**
- [x] 1.7.2 **[slint-developer]** In `src/main.rs`, add `use flashcard;` and call `flashcard::init(&ui)` after `MainWindow::new()` and before `window.run()`. `src/main.rs` must remain entry-point only. Verify the full workspace `cargo build` passes end-to-end. Suggest a commit message for the complete 1.7 change (1.7.1 + 1.7.2). **Depends on 1.7.1.**

## Phase 2: Flashcard Management

  > Tasks 2.1.1 and 2.2.1 modify different files (`study_page.slint` vs `flashcard_stack.slint`) — they may run in parallel.

- [x] 2.1.1 **[slint-developer]** In `ui/pages/study_page.slint`, add a stack-creation form that appears when the existing "Create Stack" button is clicked (use the "Vertically stacked up components pattern" from `slint-code-style.md`). The form contains a single `TextInput` for the stack name and Confirm/Cancel `CommonBtn` buttons. Add `callback stack-create-confirmed(name: string)` to `StudyPage`. No Rust logic yet — verify the form shows and hides correctly.
- [x] 2.1.2 **[rust-developer]** In `lib/flashcard/src/lib.rs`, wire `on_stack_create_confirmed`: create a new `FlashcardStackModel { stackname: name, flashcards: vec![] }`, append it to the current `flashcardList` using `VecModel`, and push it back via `set_flashcard_list`. **Depends on 2.1.1.**
- [x] 2.2.1 **[slint-developer]** In `lib/flashcard/ui/components/flashcard_stack.slint`, add an "Add Flashcard" form below the card list (use the "Vertically stacked up components pattern"). The form contains two `TextInput` fields (Japanese word, Vietnamese meaning) and Confirm/Cancel `CommonBtn` buttons. Add `callback flashcard-add-confirmed(jap: string, meaning: string)` to `FlashcardStack`. Verify the form shows and hides correctly. **Independent of 2.1.1 — may run in parallel.**
- [x] 2.2.2 **[rust-developer]** Wire `on_flashcard_add_confirmed`: create a new `FlashcardModel`, append it to the selected stack's `flashcards` using `VecModel`, and push the updated `flashcardList` back. **Depends on 2.2.1 and 2.1.2.**

  > Tasks 2.3.1 and 2.4.1 both modify `flashcard_stack.slint` — implement them sequentially.

- [x] 2.3.1 **[slint-developer]** In `lib/flashcard/ui/components/flashcard_stack.slint`, wire the existing `TextInput` fields for editing: add `edited` callbacks on both `jap-obj` and `explanation` fields using `for card[i]` index. Add `callback flashcard-field-changed(card-index: int, jap: string, meaning: string)` to `FlashcardStack`. Wire the callback through `study_page.slint` to `FlashcardAppLogic`. **Depends on 2.2.1.**
- [x] 2.3.2 **[rust-developer]** Wire `on_flashcard_field_changed`: replace the `FlashcardModel` at `card-index` in the selected stack with updated field values, then push the refreshed `flashcardList` back. **Depends on 2.3.1.**
- [x] 2.4.1 **[slint-developer]** In `lib/flashcard/ui/components/flashcard_stack.slint`, add a delete `CommonBtn` ("✕") per card row inside the `for card[i]` loop. Add `callback flashcard-delete-confirmed(card-index: int)` to `FlashcardStack`. Wire through `study_page.slint` to `FlashcardAppLogic`. **Depends on 2.3.1.**
- [x] 2.4.2 **[rust-developer]** Wire `on_flashcard_delete_confirmed`: remove the `FlashcardModel` at `card-index` from the selected stack and push the updated `flashcardList` back. **Depends on 2.4.1.**
- [x] 2.5 Verify the complete CRUD workflow on a running build: create a stack, add cards, edit fields, delete cards. Confirm `FlashcardStack` reflects all changes live without re-selection. No new code unless a data-binding bug is found. **Depends on 2.1.2, 2.2.2, 2.3.2, 2.4.2.**

  > Tasks 2.6.1/2.6.2 (drag-to-reorder) and 2.6.3/2.6.4 (delete stack) all touch the same files
  > (`flashcard_stack.slint` and `lib.rs`), so the full chain is strictly sequential:
  > 2.6.1 → 2.6.2 → 2.6.3 → 2.6.4.

- [x] 2.6.1 **[slint-developer]** In `lib/flashcard/ui/components/flashcard_stack.slint`, implement drag-to-reorder for card rows using Slint pointer/touch events. Add `callback flashcard-reordered(from-index: int, to-index: int)` to `FlashcardStack`. **Depends on 2.5.**
- [x] 2.6.2 **[rust-developer]** Wire `on_flashcard_reordered`: swap the `FlashcardModel` entries at `from-index` and `to-index` in the selected stack, then push the updated `flashcardList` back. **Depends on 2.6.1.**
- [x] 2.6.3 **[slint-developer]** In `lib/flashcard/ui/components/flashcard_stack.slint`, add a "Delete Stack" `CommonBtn` to the stack header row (beside the existing close "✕" button). Add `callback stack-delete-confirmed` to `FlashcardStack`. In `ui/pages/study_page.slint`, wire `stack-delete-confirmed` → `FlashcardAppLogic.stack-delete-confirmed()`. `FlashcardAppLogic` already declares `callback stack-delete-confirmed()` — no change needed there. **Depends on 2.6.2.**
- [x] 2.6.4 **[rust-developer]** Wire `on_stack_delete_confirmed` in `lib/flashcard/src/lib.rs`: remove the `FlashcardStackModel` at `selected-stack-index` from `flashcard-list`, reset `selected-stack-index = -1`, and push the updated list back. **Depends on 2.6.3.**
- [x] 2.7 **[rust-developer]** Implement session persistence: on every `flashcardList` change serialize it to a local `stacks.json` file using `serde` + `serde_json`; load and restore it at application startup. Add `serde` and `serde_json` to `lib/flashcard/Cargo.toml`. Define shadow Rust structs `StackData`/`CardData` with `#[derive(Serialize, Deserialize)]`; convert to/from Slint types inside `lib.rs`. Gate all `std::fs` calls with `#[cfg(not(target_arch = "wasm32"))]`. Call `save_stacks()` after every `set_flashcard_list` in each CRUD handler; call `load_stacks()` at the start of `init()`. **Depends on 2.4.2.**
- [x] 2.8 Test flashcard CRUD operations and persistence manually on Windows: verify data survives application restart. **Depends on 2.7.**
- [x] 2.9 **[slint-tester]** Create automation CRUD operation tests for `StudyPage` following the format in `.claude/rules/slint-test-format.md`. Cover: `study_page_create_stack`, `study_page_create_card`, `study_page_read_list`, `study_page_update_card`, `study_page_reorder_cards`, `study_page_delete_card`, `study_page_delete_stack`, `study_page_persistence_round_trip`. Steps: (1) Add `FlashcardTestWindow` component to `lib/flashcard/ui/flashcard_lib.slint`. (2) Create `test/flashcard/` as a workspace member: `Cargo.toml` (name = "flashcard-tests", depends on `flashcard` crate and `slint` with `features = ["testing"]`), add `"test/flashcard"` to workspace `members` in root `Cargo.toml`. (3) Write all tests in `test/flashcard/tests/study_page.rs` using the templates in `.claude/rules/slint-test-format.md`. (4) Run `cargo test -p flashcard-tests` and confirm all tests pass. (5) Run `cargo fmt` and `cargo clippy` with zero warnings. **Depends on 2.8.**

## Phase 3: Universal Styling Library

**Goal**: Extract all design tokens into a standalone `lib/styles` folder (libC — pure Slint, no Rust backend) so every component in every library imports from `@styles`. No hardcoded colors, sizes, or durations anywhere.

  > Tasks 3.1 → 3.2 → 3.3 → 3.4 are strictly sequential — each depends on the previous.

- [ ] 3.1 **[slint-developer]** Create `lib/styles/` as a pure Slint libC folder (**no** `Cargo.toml`, **no** `src/`, **no** `build.rs`). Create three files:
  - `lib/styles/tokens.slint`: expand the existing token set from `lib/flashcard/ui/styles/tokens.slint` with any missing values: button state colors (default, hover, pressed, disabled), known/unknown indicator colors, page background, nav bar background, input border colors, font weights.
  - `lib/styles/animations.slint`: easing-curve constants (`ease-in`, `ease-out`, `ease-in-out`) and duration tokens (`flip-duration: 150ms`, `transition-duration: 200ms`).
  - `lib/styles/styles.slint`: entry file — re-exports `{ Tokens }` from `"tokens.slint"` and `{ Animations }` from `"animations.slint"`. This is the file clients register as `@styles`.

- [ ] 3.2 **[slint-developer]** Register `@styles` library path in all `build.rs` files that compile `.slint` sources:
  - `lib/flashcard/build.rs`: chain `.with_library_paths(...)` onto the existing `CompilerConfiguration` — keep `as_library("flashcard").rust_module("flashcard")`; add the `styles` key pointing to `../../lib/styles/styles.slint` (relative from `CARGO_MANIFEST_DIR`).
  - Root `build.rs`: switch from `slint_build::compile(...)` to `slint_build::compile_with_config(...)` and pass `with_library_paths` pointing to `lib/styles/styles.slint`.
  - Verify `cargo build` passes after this change alone — no import changes yet. **Depends on 3.1.**

- [ ] 3.3 **[slint-developer]** Migrate all `.slint` files to import from `@styles`:
  - In every file under `lib/flashcard/ui/` that imports `tokens.slint` locally, replace the local import with `import { Tokens, Animations } from "@styles"`.
  - In every file under root `ui/` that imports local style files, do the same.
  - Delete `lib/flashcard/ui/styles/tokens.slint` — its content is now owned by `lib/styles/tokens.slint`.
  - Verify `cargo build` passes — all `@styles` imports resolve correctly. **Depends on 3.2.**

- [ ] 3.4 **[slint-developer]** Audit all `.slint` files for remaining hardcoded colors (e.g., `#rrggbb`, `Colors.x`), hardcoded pixel sizes (e.g., `16px` where a token applies), or hardcoded durations (e.g., `150ms` where an `Animations` token applies). Replace every occurrence with the appropriate `Tokens.*` or `Animations.*` reference. Add new token entries to `lib/styles/tokens.slint` only when a value is genuinely missing — do not over-tokenize layout dimensions (e.g., explicit padding values in layouts are fine). Verify `cargo build` passes with zero warnings. **Depends on 3.3.**

## Phase 4: Study Mode

**Goal**: Deliver a functional single-card study session wired to live Rust-computed progress. Depends on Phase 3 (`@styles` tokens must be available before implementing study mode UI).

- [ ] 4.1 **[slint-developer]** In `ui/pages/study_page.slint`, add a study session view that activates when the user clicks a "Study" `CommonBtn` on the open `FlashcardStack`. The session displays one `Flashcard` component at a time (from task 1.3.3) centered on screen, with "Previous" and "Next" `CommonBtn` buttons and a `callback study-session-closed` to return to the stack list. Track `current-card-index` as an `in-out property <int>`. Use the existing `Flashcard` component — do not re-implement flip or known toggle. Apply the "Vertically stacked up components pattern" to manage visibility. **Depends on Phase 3 completion.**

  > Tasks 4.2 and 4.3 both depend on 4.1 but are independent of each other — they may run in parallel.

- [ ] 4.2 **[slint-developer]** In the study session view, verify the reveal mechanism works end-to-end: the `Flashcard` component's `show-back` property (tap-to-flip, from task 1.3.3) shows the Japanese word first and hides the Vietnamese meaning until tapped. Confirm "Tap to reveal / Tap to hide" hint text updates correctly. No new Slint code is needed if `Flashcard` is wired correctly in 4.1 — this is a verification and any wiring fix task. **Depends on 4.1.**
- [ ] 4.3 **[slint-developer]** In the study session view, verify the known/unknown toggle works: the `Flashcard` component's `checkable: true; checked <=> known` binding (from task 1.3.4) handles marking in both directions. Confirm the toggle icon (✓/✗) and card background color update correctly. **Depends on 4.1.**

  > Tasks 4.4 and 4.5 are sequential.

- [ ] 4.4 **[slint-developer]** In `ui/pages/study_page.slint`, add a progress display to the study session view showing known vs total counts (e.g., "3 / 5 known"). Add `in property <int> known-count: 0` and `in property <int> total-count: 0` to the session component. Bind the display to these properties; use hardcoded placeholder values to verify the UI renders correctly. **Depends on 4.3.**
- [ ] 4.5 **[rust-developer]** Wire live progress counts in `lib/flashcard/src/lib.rs`: compute `known-count` by iterating the active stack's `flashcards` and counting `known == true`; set `total-count` to the stack size. Expose `callback known-changed` on `FlashcardAppLogic` if not already present. Recompute and push both counts to `StudyPage` via a weak handle whenever the known status changes. **Depends on 4.4.**
- [ ] 4.6 Test study mode interactions manually on Windows: flip cards, toggle known status, navigate prev/next, verify progress counts update in real time, verify session close returns to stack list. **Depends on 4.5.**

## Phase 5: Persistent Data Management
- [ ] 5.1 Define the markdown file format specification for flashcard stacks. The format uses `## Stack Name` headings to delimit stacks and a GFM pipe table (`| Japanese | Meaning |`) under each heading for cards. Document the format with a worked example in `docs/markdown-format.md`. This is a prerequisite for all tasks in this phase; complete it before starting any other Phase 5 task.

  > Tasks 5.2.1 and 5.2.2 are independent of each other — they may be started in parallel after 5.1.

- [ ] 5.2.1 **[rust-developer]** Create the `lib/persistent_data/` workspace member following the **libB** (Rust service library) pattern from the constitution. Create `lib/persistent_data/Cargo.toml` with `name = "persistent_data"`, `edition = "2024"`, and dependencies `pulldown-cmark = "0.12"` and `rfd = { version = "0.15" }`. Create an empty `lib/persistent_data/src/lib.rs`. Add `"lib/persistent_data"` to workspace `members` in root `Cargo.toml` and add `persistent_data = { path = "lib/persistent_data" }` to root `[dependencies]`. Verify `cargo build` passes — no functional code yet.
- [ ] 5.2.2 **[slint-developer]** Add `callback import-stack-clicked` and `callback export-stack-clicked` to `StudyPage` in `ui/pages/study_page.slint`. Wire an "Import" `CommonBtn` and an "Export" `CommonBtn` into the existing page header following the `*Btn` naming convention. No Rust logic yet. Verify the build passes and both buttons are visible. **This task has no Rust dependency — it may run in parallel with 5.2.1 and 5.3.x; it only needs to complete before 5.4.**

  > Tasks 5.3.1 and 5.3.2 both depend on 5.2.1 but are independent of each other — they may be started in parallel once 5.2.1 is complete.

- [ ] 5.3.1 **[rust-developer]** Implement `lib/persistent_data/src/markdown_io.rs`. Define `StackData { name: String, cards: Vec<CardData> }` and `CardData { japanese: String, meaning: String }` as plain Rust structs (no Slint types). Implement `parse_stacks(source: &str) -> Vec<StackData>` using `pulldown-cmark` with `Options::ENABLE_TABLES` per the format in task 5.1, and `serialize_stacks(stacks: &[StackData]) -> String` that round-trips through the parser. Expose the module in `lib.rs`. Add unit tests: single stack with two cards, two stacks, empty input, and a stack with no cards. **Depends on 5.2.1.**
- [ ] 5.3.2 **[rust-developer]** Implement `lib/persistent_data/src/file_io.rs` with `open_markdown_file() -> Option<String>` (open dialog + `read_to_string`) and `save_markdown_file(content: &str) -> bool` (save dialog + `write`). Use `rfd::FileDialog` (sync API, `*.md` filter). Gate the entire module body with `#[cfg(not(target_arch = "wasm32"))]`; provide no-op stubs (`None` / `false`) under `#[cfg(target_arch = "wasm32")]`. Expose the module in `lib.rs`. Verify `cargo build` passes on both Windows and WASM targets. **Depends on 5.2.1.**

  > Task 5.4 requires 5.2.2, 5.3.1, and 5.3.2 to all be complete before it can start.

- [ ] 5.4 **[rust-developer]** Implement the `init()` function in `lib/persistent_data/src/lib.rs`. Accept `&MainWindow` and register two callback handlers. `on_import_stack_clicked`: call `file_io::open_markdown_file()` → `markdown_io::parse_stacks()` → convert each `StackData` to `FlashcardStackModel` (SharedString fields, VecModel for cards) → push to `StudyPage`'s `flashcard-list` via a weak handle. `on_export_stack_clicked`: read `flashcard-list` → convert to `Vec<StackData>` → `markdown_io::serialize_stacks()` → `file_io::save_markdown_file()`. All Slint↔Rust type conversions stay in `lib.rs`; `markdown_io` and `file_io` remain Slint-free. **Depends on 5.2.2, 5.3.1, and 5.3.2.**
- [ ] 5.5 **[rust-developer]** Call `persistent_data::init(&ui)` in `src/main.rs` after `MainWindow::new()`, following the same init-call pattern as the flashcard library from task 1.7. `src/main.rs` must remain entry-point only. Verify on Windows: importing a `.md` file populates the stack list; exporting produces a file that round-trips back through import without data loss. **Depends on 5.4.**

## Phase 6: Optimization & Testing
- [ ] 6.1 Optimize rendering performance for Windows and WebAssembly targets.
- [ ] 6.2 Test UI responsiveness across both targets.
- [ ] 6.3 Add Rust unit tests for core logic (flashcard CRUD, study mode state).
- [ ] 6.4 Add Rust integration tests for data persistence.
- [ ] 6.5 Ensure compliance with constitution best practices (UI separation, modularity).
- [ ] 6.6 Document testing results and performance benchmarks.

## Phase 7: Future Backlog (Extensible)
- [ ] 7.1 Add audio playback (Japanese text‑to‑speech integration).
- [ ] 7.2 Implement spaced repetition algorithms for study scheduling.
- [ ] 7.3 Add synchronization across devices (future cloud sync).
- [ ] 7.4 Add WASM-compatible import/export: replace `rfd` + `std::fs` with a browser file-input element via `web-sys` and JavaScript interop. Gate with `#[cfg(target_arch = "wasm32")]`.
- [ ] 7.5 Add analytics and reporting features (progress charts, study statistics).

# Deliverables
- Each task produces incremental functionality aligned with the plan.
- Completion of all tasks results in:
  - A working Windows + WebAssembly application with flashcard management, study mode, and markdown-based import/export.
  - Documentation of architecture, modules, and usage.
  - Rust test suite covering core features.
  - Backlog items prepared for future iterations.
