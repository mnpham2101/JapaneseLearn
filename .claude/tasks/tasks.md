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

**Goal**: Extract all design tokens into a standalone `lib/styles` folder (libC — pure Slint, no Rust backend) so every component in every library imports from `@styles`. No hardcoded colors, sizes, or durations anywhere. Each task is one atomic commit that builds independently.

- [x] 3.1 **[slint-developer]** Create `lib/styles/` as a pure Slint libC folder (**no** `Cargo.toml`, **no** `src/`, **no** `build.rs`). Three files only — no client imports @styles yet, so the build is unaffected:
  - `lib/styles/tokens.slint`: expand the existing token set from `lib/flashcard/ui/styles/tokens.slint` — add button state colors (default, hover, pressed, disabled), known/unknown indicator colors, page/nav backgrounds, input field colors, stack label colors.
  - `lib/styles/animations.slint`: `flip-duration: 150ms`, `transition-duration: 200ms`, easing constants.
  - `lib/styles/styles.slint`: entry file re-exporting `{ Tokens }` from `"tokens.slint"` and `{ Animations }` from `"animations.slint"`.
  - **Atomic commit**: "feat: create lib/styles libC folder with design tokens"

  > Tasks 3.2.1 and 3.2.2 are independent — may run in parallel. Both depend on 3.1.

- [x] 3.2.1 **[slint-developer]** Register `@styles` in `lib/flashcard/build.rs` only. Chain `.with_library_paths(styles → ../../lib/styles/styles.slint)` onto the existing `as_library / rust_module` config. No `.slint` file imports `@styles` yet — build passes unchanged. **Depends on 3.1.**
  - **Atomic commit**: "build: register @styles path in flashcard build.rs"

- [x] 3.2.2 **[slint-developer]** Register `@styles` in root `build.rs` only. Switch `slint_build::compile(...)` to `compile_with_config(...)` with `with_library_paths(styles → lib/styles/styles.slint)`. No root `.slint` file imports `@styles` yet — build passes. **Depends on 3.1.**
  - **Atomic commit**: "build: register @styles path in root build.rs"

- [x] 3.3 **[slint-developer]** Migrate `lib/flashcard/ui/` Slint files to `@styles`. Replace every local `import { Tokens } from "../styles/tokens.slint"` (or any relative path to the local tokens file) with `import { Tokens } from "@styles"`. Delete `lib/flashcard/ui/styles/tokens.slint` — its content now lives in `lib/styles/tokens.slint`. Verify build passes. **Depends on 3.2.1.**
  - **Atomic commit**: "refactor: migrate flashcard library .slint files to @styles"

- [x] 3.4 **[slint-developer]** Migrate root `ui/` Slint files to `@styles`. Replace any local style imports in `ui/pages/`, `ui/components/`, `ui/main_window.slint` with `import { Tokens } from "@styles"`. Verify build passes. **Depends on 3.2.2.**
  - **Atomic commit**: "refactor: migrate root UI .slint files to @styles"

- [x] 3.5 **[slint-developer]** Audit all `.slint` files for remaining hardcoded colors (`#rrggbb`, `Colors.x`), pixel sizes where a token applies, or durations where an `Animations` token applies. Replace every occurrence with the appropriate `Tokens.*` or `Animations.*` reference. Add new token entries to `lib/styles/tokens.slint` only when a value is genuinely missing — do not over-tokenize layout dimensions. Verify `cargo build` and `cargo clippy` pass with zero warnings. **Depends on 3.3 and 3.4.**
  - **Atomic commit**: "refactor: replace remaining hardcoded style values with @styles tokens"

## Phase 4: Study Mode

**Goal**: Deliver a functional single-card study session wired to live Rust-computed progress counts. Depends on Phase 3. Each task is one atomic commit that builds independently.

- [x] 4.1 **[slint-developer]** Add study session state to `FlashcardAppLogic` in `lib/flashcard/ui/flashcard_app_logic.slint`. Declarations only — no UI changes yet, no Rust handler needed:
  - `in-out property <bool> study-session-active: false`
  - `in property <int> known-count: 0`
  - `in property <int> total-count: 0`
  - `callback known-changed(stack-index: int, card-index: int, known: bool)`
  - Verify build passes. **Depends on Phase 3 completion.**
  - **Atomic commit**: "feat: add study session state properties to FlashcardAppLogic"

- [x] 4.2 **[slint-developer]** Add "Study" button to `FlashcardStack` in `lib/flashcard/ui/components/flashcard_stack.slint`. Add `callback study-clicked` to `FlashcardStack`. In `ui/pages/study_page.slint`, wire `study-clicked` → `FlashcardAppLogic.study-session-active = true` and reset `current-card-index = 0` (declare `property <int> current-card-index: 0` on `StudyPage`). No Rust handler needed for study-session-active — it is a pure Slint property. Verify build passes. **Depends on 4.1.**
  - **Atomic commit**: "feat: add Study button to FlashcardStack and wire session activation"

- [x] 4.3 **[slint-developer]** Add the study session view to `StudyPage` in `ui/pages/study_page.slint`. Shown when `study-session-active == true` (use the Vertically stacked up components pattern). Contents:
  - One `Flashcard` component bound to `flashcardList[selected-stack-index].flashcards[current-card-index]` — tap-to-flip and known toggle work automatically via existing Flashcard bindings.
  - "Previous" and "Next" `CommonBtn` buttons, bounds-clamped to `[0, stack.flashcards.row_count - 1]`.
  - Close `CommonBtn` that sets `study-session-active = false`.
  - Progress `Text` bound to `FlashcardAppLogic.known-count` and `FlashcardAppLogic.total-count` (declared in 4.1): e.g., `known-count + " / " + total-count + " known"`.
  - Inside the `Flashcard`, wire `changed known` → `FlashcardAppLogic.known-changed(selected-stack-index, current-card-index, card.known)` so Rust can react.
  - Verify build passes (Rust `on_known_changed` handler is not yet registered — that is fine). **Depends on 4.2.**
  - **Atomic commit**: "feat: add study session view with navigation and progress display"

- [x] 4.4 **[rust-developer]** Add `update_progress()` helper to `lib/flashcard/src/lib.rs`. This function reads `selected-stack-index` from the logic, iterates the active stack's flashcards, counts `known == true`, and sets `logic.set_known_count(known)` / `logic.set_total_count(total)`. This commit will produce a dead-code warning (function not yet called) — that is acceptable per `atomic-commit-rule.md`; the warning clears in task 4.5. Verify `cargo build` passes (zero errors) and all 8 tests pass. Do not add artificial calls to `init()` to suppress the warning. **Depends on 4.3.**
  - **Atomic commit**: "feat: add update_progress helper for study session counts"

- [x] 4.5 **[rust-developer]** Register `on_known_changed` handler in `init()` in `lib/flashcard/src/lib.rs`. The handler receives `(stack_index, card_index, known)` from the Slint callback (wired in 4.3). It: (1) reads `flashcard_list`, updates `card.known` at `[stack_index][card_index]`, (2) calls `save_stacks()` to persist, (3) calls `update_progress(ui)` to refresh the counts (already committed in 4.4). Verify `cargo build`, `cargo clippy` (zero warnings), and all 8 tests pass. **Depends on 4.4.**
  - **Atomic commit**: "feat: wire on_known_changed to persist known status and update progress"

- [x] 4.6 Test study mode interactions manually on Windows: open a stack → click Study → session appears; tap card → back reveals; toggle known/unknown → icon and progress update; Prev/Next navigates; Close returns to stack list; restart app → known status persisted. **Depends on 4.5.**

## Phase 5: Persistent Data Management
- [x] 5.1 Define the markdown file format specification for flashcard stacks. The format uses `## Stack Name` headings to delimit stacks and a GFM pipe table (`| Japanese | Meaning |`) under each heading for cards. Document the format with a worked example in `docs/markdown-format.md`. This is a prerequisite for all tasks in this phase; complete it before starting any other Phase 5 task.

  > Tasks 5.2.1 and 5.2.2 are independent of each other — they may be started in parallel after 5.1.

- [x] 5.2.1 **[rust-developer]** Create the `lib/persistent_data/` workspace member following the **libB** (Rust service library) pattern from the constitution. Create `lib/persistent_data/Cargo.toml` with `name = "persistent_data"`, `edition = "2024"`, and dependencies `pulldown-cmark = "0.12"` and `rfd = { version = "0.15" }`. Create an empty `lib/persistent_data/src/lib.rs`. Add `"lib/persistent_data"` to workspace `members` in root `Cargo.toml` and add `persistent_data = { path = "lib/persistent_data" }` to root `[dependencies]`. Verify `cargo build` passes — no functional code yet.
- [x] 5.2.2 **[slint-developer]** Add `callback import-stack-clicked` and `callback export-stack-clicked` to `StudyPage` in `ui/pages/study_page.slint`. Wire an "Import" `CommonBtn` and an "Export" `CommonBtn` into the existing page header following the `*Btn` naming convention. No Rust logic yet. Verify the build passes and both buttons are visible. **This task has no Rust dependency — it may run in parallel with 5.2.1 and 5.3.x; it only needs to complete before 5.4.**

  > Tasks 5.3.1 and 5.3.2 both depend on 5.2.1 but are independent of each other — they may be started in parallel once 5.2.1 is complete.

- [x] 5.3.1 **[rust-developer]** Implement `lib/persistent_data/src/markdown_io.rs`. Define `StackData { name: String, cards: Vec<CardData> }` and `CardData { japanese: String, meaning: String }` as plain Rust structs (no Slint types). Implement `parse_stacks(source: &str) -> Vec<StackData>` using `pulldown-cmark` with `Options::ENABLE_TABLES` per the format in task 5.1, and `serialize_stacks(stacks: &[StackData]) -> String` that round-trips through the parser. Expose the module in `lib.rs`. Add unit tests: single stack with two cards, two stacks, empty input, and a stack with no cards. **Depends on 5.2.1.**
- [x] 5.3.2 **[rust-developer]** Implement `lib/persistent_data/src/file_io.rs` with `open_markdown_file() -> Option<String>` (open dialog + `read_to_string`) and `save_markdown_file(content: &str) -> bool` (save dialog + `write`). Use `rfd::FileDialog` (sync API, `*.md` filter). Gate the entire module body with `#[cfg(not(target_arch = "wasm32"))]`; provide no-op stubs (`None` / `false`) under `#[cfg(target_arch = "wasm32")]`. Expose the module in `lib.rs`. Verify `cargo build` passes on both Windows and WASM targets. **Depends on 5.2.1.**

  > Task 5.4 requires 5.2.2, 5.3.1, and 5.3.2 to all be complete before it can start.

- [x] 5.4 **[rust-developer]** Implement the `init()` function in `lib/persistent_data/src/lib.rs`. Accept `&MainWindow` and register two callback handlers. `on_import_stack_clicked`: call `file_io::open_markdown_file()` → `markdown_io::parse_stacks()` → convert each `StackData` to `FlashcardStackModel` (SharedString fields, VecModel for cards) → push to `StudyPage`'s `flashcard-list` via a weak handle. `on_export_stack_clicked`: read `flashcard-list` → convert to `Vec<StackData>` → `markdown_io::serialize_stacks()` → `file_io::save_markdown_file()`. All Slint↔Rust type conversions stay in `lib.rs`; `markdown_io` and `file_io` remain Slint-free. **Depends on 5.2.2, 5.3.1, and 5.3.2.**
- [x] 5.5 **[rust-developer]** Call `persistent_data::init(&ui)` in `src/main.rs` after `MainWindow::new()`, following the same init-call pattern as the flashcard library from task 1.7. `src/main.rs` must remain entry-point only. Verify on Windows: importing a `.md` file populates the stack list; exporting produces a file that round-trips back through import without data loss. **Depends on 5.4.**

## Phase 6: Vocabulary Study Mode and Exercise Generation
**Goal**: Users can author vocabulary lessons, generate decoupled flashcard stacks on demand. Review Page gains a read-only matching exercise.

  > Tasks 6.1 and 6.6 are independent — may run in parallel.

- [x] 6.1 **[rust-developer]** Create `lib/exercise_generator` libD scaffold: `Cargo.toml` (`name = "exercise_generator"`, `serde` dep), empty `src/lib.rs`, add `"lib/exercise_generator"` to workspace `members` in root `Cargo.toml`. No functional code yet. Verify `cargo build` passes. **Independent of 6.6 — may run in parallel.**
- [x] 6.2 **[rust-developer]** Implement `lib/exercise_generator/src/models.rs`: `VocabularyLesson`, `VocabularyWord`, `TenseEntry`, `FlashcardStackData`, `FlashcardCardData` plain Rust structs per `.claude/rules/libD-code-style.md §Domain Models`. Re-export from `lib.rs`. **Depends on 6.1.**
- [x] 6.3 **[rust-developer]** Implement `lib/exercise_generator/src/transformer.rs`: `Transformer<S,T>` trait, `ExerciseRequest` enum, `ExerciseOutput` enum per `.claude/rules/libD-code-style.md §Transformer Trait`. Re-export from `lib.rs`. **Depends on 6.2.**
- [x] 6.4 **[rust-developer]** Implement `lib/exercise_generator/src/service.rs`: `ExerciseGeneratorFor<S>` trait and `ExerciseGeneratorService` dispatcher per `.claude/rules/libD-code-style.md §Service Dispatcher`. Re-export from `lib.rs`. **Depends on 6.3.**
- [x] 6.5 **[rust-developer]** Implement `lib/exercise_generator/src/flashcard_transformer.rs`: `FlashcardExerciseTransformer` with kanji duplication rule + inline `#[cfg(test)]` unit tests (4 cases). Run `cargo test -p exercise_generator`. **Depends on 6.4.**

  > Task 6.6 may run in parallel with 6.1–6.5. Tasks 6.11 and 6.12 both depend only on 6.6 and may run in parallel with each other after 6.6 is complete.

- [x] 6.6 **[slint-developer]** Create `lib/vocabulary` libA scaffold: `Cargo.toml` (`name = "vocabulary"`, `slint` + `flashcard` workspace deps), `build.rs`, `src/lib.rs` init stub, `ui/vocabulary_lib.slint` entry file, `VocabularyAppLogic` global, `VocabularyLessonModel` and `VocabularyWordModel` Slint structs. Add to workspace. Verify `cargo build` passes. **Independent of 6.1–6.5 — may run in parallel.**
- [x] 6.7 **[slint-developer]** Vocabulary lesson list UI in `lib/vocabulary/ui/`.
  - [x] 6.7.1 Add lesson CRUD callbacks to `VocabularyAppLogic` — see [6-7-1.md](.claude/tasks/subtask/6-7-1.md)
  - [x] 6.7.2 Implement `LessonList` component — see [6-7-2.md](.claude/tasks/subtask/6-7-2.md)

  **Depends on 6.6.**
- [x] 6.8 **[slint-developer]** Vocabulary word form UI in `lib/vocabulary/ui/`.
  - [x] 6.8.1 Word form: spelling, kanji, meaning, type fields — see [6-8-1.md](.claude/tasks/subtask/6-8-1.md)
  - [x] 6.8.2 Word form: tense list and example list — see [6-8-2.md](.claude/tasks/subtask/6-8-2.md)

  **Depends on 6.7.**
- [x] 6.9 **[rust-developer]** `lib/vocabulary` Rust backend: persistence, CRUD handlers, vocabulary markdown import/export.
  - [x] 6.9.1 Persistence: `load_vocabulary()` / `save_vocabulary()` for `vocabulary.json` — see [6-9-1.md](.claude/tasks/subtask/6-9-1.md)
  - [x] 6.9.2 CRUD handlers for lessons and words wired in `init()` — see [6-9-2.md](.claude/tasks/subtask/6-9-2.md)
  - [x] 6.9.3 Vocabulary markdown import/export (`vocabulary_markdown_io.rs`) — see [6-9-3.md](.claude/tasks/subtask/6-9-3.md)

  **Depends on 6.8.**
- [x] 6.10 **[rust-developer]** Wire `on_generate_exercises_clicked` in `lib/vocabulary/src/lib.rs`: convert `VocabularyAppLogic` state → `Vec<VocabularyLesson>`, call `ExerciseGeneratorService::generate(ExerciseRequest::Flashcard, &lessons)`, convert output → `FlashcardStackModel`, update `FlashcardAppLogic` and call `flashcard::save_stacks()`. **Depends on 6.5 and 6.9.**

  > Tasks 6.11 and 6.12 may run in parallel — both depend only on 6.6.

- [x] 6.11 **[slint-developer]** Add topic selector to `StudyPage` in `ui/pages/study_page.slint`: "Vocabulary" tab shows `VocabularyPage`; existing flashcard stack view moves under "Flashcard" tab; Grammar and Reading are placeholder tabs. **Depends on 6.6.**
- [x] 6.12 **[slint-developer]** Implement `MatchingExerciseView` in `lib/vocabulary/ui/components/`: card tiles with front/back text, click-to-select and click-to-match logic, locked visual state per matched pair, `callback exercise-completed`. **Depends on 6.6.**
- [x] 6.13 **[slint-developer]** Update `ReviewPage` in `ui/pages/review_page.slint`: show flashcard stack list in read-only mode; selecting a stack launches `MatchingExerciseView`. **Depends on 6.12.**
- [x] 6.14 **[rust-developer]** Call `vocabulary::init(&ui)` in `src/main.rs` after `flashcard::init` and `persistent_data::init`. `src/main.rs` must remain entry-point only. **Depends on 6.10.**
- [x] 6.T **[slint-tester]** Test vocabulary CRUD on `VocabularyPage`: create lesson, add word, update word, delete word, delete lesson, persistence round-trip.
  - Callbacks to invoke: `invoke_lesson_create_confirmed(name)`, `invoke_word_add_confirmed(lesson_idx, spelling, kanji, meaning)`, `invoke_word_field_changed(lesson_idx, word_idx, ...)`, `invoke_word_delete_confirmed(lesson_idx, word_idx)`, `invoke_lesson_delete_confirmed(lesson_idx)`.
  - Properties to assert: `get_lesson_list().row_count()`, `get_lesson_list().row_data(0).words.row_count()`.
  - Covers: Task 6.7 + 6.8 + 6.9.
  **Depends on 6.9.**

## Phase 6.R: UI Redesign (post Phase 6)
**Goal**: Move `StudySessionView` into `lib/flashcard`, build new vocabulary-side components (`LessonStackLabel`, `LessonDetailView`, `LessonStackList`, `FlashcardManagerView`), redesign `VocabularyPage` with an action-bar tab layout, and simplify `StudyPage` to three tabs.

> Tasks 6.R.1, 6.R.2, and 6.R.3 are independent — implement sequentially for safety (single agent, no file conflicts).

- [x] 6.R.1 **[slint-developer]** Move `StudySessionView` to `lib/flashcard/ui/components/`, export from `flashcard_lib.slint`, update `study_page.slint` import. — see [6-R-1.md](.claude/tasks/subtask/6-R-1.md)
- [x] 6.R.2 **[slint-developer]** Create `LessonStackLabel` in `lib/vocabulary/ui/components/lesson_stack_label.slint`; export from `vocabulary_lib.slint`. — see [6-R-2.md](.claude/tasks/subtask/6-R-2.md)
- [x] 6.R.3 **[slint-developer]** Create `LessonDetailView` in `lib/vocabulary/ui/components/lesson_detail_view.slint`; export from `vocabulary_lib.slint`. — see [6-R-3.md](.claude/tasks/subtask/6-R-3.md)
- [x] 6.R.4 **[slint-developer]** Create `LessonStackList` in `lib/vocabulary/ui/components/lesson_stack_list.slint`; export from `vocabulary_lib.slint`. **Depends on 6.R.2.** — see [6-R-4.md](.claude/tasks/subtask/6-R-4.md)
- [x] 6.R.5 **[slint-developer]** Create `FlashcardManagerView` in `lib/vocabulary/ui/components/flashcard_manager_view.slint`; export from `vocabulary_lib.slint`. **Depends on 6.R.1.** — see [6-R-5.md](.claude/tasks/subtask/6-R-5.md)
- [x] 6.R.6 **[slint-developer]** Redesign `VocabularyPage` with action-bar tabs (Lesson/Exercise/Flashcard) and Import Lesson action. **Depends on 6.R.3, 6.R.4, 6.R.5.** — see [6-R-6.md](.claude/tasks/subtask/6-R-6.md)
- [x] 6.R.7 **[slint-developer]** Simplify `StudyPage`: remove Flashcard tab, re-index to Vocabulary=0/Grammar=1/Reading=2, remove flashcard management block and unused imports. **Depends on 6.R.6.**

## Phase 6.S: Style and UX Refinements (pre-Milestone 3)
**Goal**: Apply the new day-mode color palette, make tab bars visually distinct, add flashcard flip animation, add kanji-card calligraphic rendering, fix layout bugs, and make all views responsive. All tasks target the current branch `feature/vocabulary-page-redesign`.

> 6.S.1 must be completed first. 6.S.2 and 6.S.5 both depend on 6.S.1 and may run in parallel after it. 6.S.3 depends on 6.S.1 and leads into the sequential chain 6.S.3 → 6.S.4.1 → 6.S.4.2.

- [x] 6.S.1 **[slint-developer]** Apply new day-mode color palette (#CCD5AE/#E9EDC9/#FEFAE0/#FAEDCD/#D4A373/#251504) to `tokens.slint`; add `topic-tab-bar-bg`, `action-bar-bg`, and `action-tab-btn-active-bg` tokens; update bar backgrounds in `study_page.slint` and `vocabulary_page.slint`. — see [6-S-1.md](.claude/tasks/subtask/6-S-1.md)

- [x] 6.S.2 **[slint-developer]** Improve `CommonBtn`: add `checked-bg` in-property (defaulting to `Tokens.nav-background`) for per-instance active color; update background expression to use `btn-pressed-bg` on press; set `visible: enabled` to hide disabled buttons from layout; apply `checked-bg: Tokens.action-tab-btn-active-bg` on the three VocabularyPage action-bar tab buttons. **Depends on 6.S.1.** — see [6-S-2.md](.claude/tasks/subtask/6-S-2.md)

- [x] 6.S.3 **[slint-developer]** Implement flashcard card-flip animation in `flashcard.slint` using the width-compression trick (front collapses ease-in, back expands ease-out with delay); add `flip-delay` token to `animations.slint`; import `Animations` in `flashcard.slint`. **Depends on 6.S.1.** — see [6-S-3.md](.claude/tasks/subtask/6-S-3.md)

- [x] 6.S.4 **[slint-developer + rust-developer]** Kanji flashcard large-font rendering — add `is-kanji` field and calligraphic styling.
  - [x] 6.S.4.1 **[slint-developer]** Add `is-kanji: bool` to `FlashcardModel` Slint struct; add `font-size-kanji: 72px` to `tokens.slint`; render front face with extra-large MS Mincho font when `is-kanji` is true. **Depends on 6.S.3.** — see [6-S-4-1.md](.claude/tasks/subtask/6-S-4-1.md)
  - [x] 6.S.4.2 **[rust-developer]** Add `is_kanji: bool` to `FlashcardCardData` in `exercise_generator`; update `FlashcardExerciseTransformer` to set `is_kanji: true` on kanji-front cards; add `is_kanji` (serde default=false) to `CardData` in `flashcard/src/lib.rs` and update conversion functions; update vocabulary `lib.rs` type conversion. **Depends on 6.S.4.1.** — see [6-S-4-2.md](.claude/tasks/subtask/6-S-4-2.md)

- [x] 6.S.5 **[slint-developer]** Fix layout bugs and responsiveness: (a) fix Confirm/Cancel overlap by removing intermediate VerticalLayout wrapper in VocabularyPage Lesson view and giving LessonStackList `height: parent.height`; (b) cap "Add Lesson" button to `min(parent.width, 200px)` inside a centering HorizontalLayout; (c) replace fixed 360px Flashcard width in StudySessionView with `min(parent.width - 32px, 480px)`. **Depends on 6.S.1.** — see [6-S-5.md](.claude/tasks/subtask/6-S-5.md)

## Phase 6.D: Default Vocabulary Data
**Goal**: Ship three N5 vocabulary datasets as embedded data files; auto-load them on first launch; add a Restore Defaults button so users can reset to the bundled content.

- [x] 6.D.1 Create default vocabulary data files in `lib/vocabulary/ui/data/`: `n5_verbs.json`, `n5_adjectives.json`, `n5_vocabulary.json` (and matching `.md` files). Schema matches `LessonData` / `WordData` structs in `lib/vocabulary/src/lib.rs`. Content: ~30 N5 verbs, ~20 N5 adjectives, ~30 N5 nouns. No agent — data authored inline.
- [x] 6.D.2 **[rust-developer]** Auto-load defaults on first launch: at the start of `init()` in `lib/vocabulary/src/lib.rs`, if `vocabulary.json` does not exist, call a new `load_and_save_defaults()` function that uses `include_str!()` to embed the three JSON files and parse+save them. Gated `#[cfg(not(target_arch = "wasm32"))]`. **Depends on 6.D.1.** — see [6-D-2.md](.claude/tasks/subtask/6-D-2.md)
- [x] 6.D.3 **[slint-developer + rust-developer]** Restore Defaults feature.
  - [x] 6.D.3.1 **[slint-developer]** Add `callback restore-defaults-clicked()` to `VocabularyAppLogic`; add `CommonBtn { text: "Restore Defaults"; }` above `LessonStackList` in the Lesson view (active-view == 0) of `vocabulary_page.slint`. **Depends on 6.D.1.** — see [6-D-3-1.md](.claude/tasks/subtask/6-D-3-1.md)
  - [x] 6.D.3.2 **[rust-developer]** Wire `on_restore_defaults_clicked` in `lib/vocabulary/src/lib.rs`: clear lesson list, reload from embedded JSON defaults via `include_str!()`, push to logic, save to `vocabulary.json`. **Depends on 6.D.3.1.** — see [6-D-3-2.md](.claude/tasks/subtask/6-D-3-2.md)

## Phase 6.B: Bug Fixes
**Goal**: Fix the Test Matching grid layout/pagination, notify the user which flashcard stacks were generated and switch to the Flashcard tab, and fix the FlashcardLabel width-overlap bug.

> 6.B.1 and 6.B.3 touch different files than the 6.B.2 chain and may run in parallel with it. 6.B.2.1 → 6.B.2.2 → 6.B.2.T are sequential (UI plumbing, then wiring, then test).

- [x] 6.B.1 **[slint-developer]** Fix `MatchingExerciseView` grid: cap front/back tile widths so both columns fit the viewport, and paginate to ~10 card pairs per page with a "Next" button. — see [6-B-1.md](.claude/tasks/subtask/6-B-1.md)
- [x] 6.B.1.T **[slint-tester]** Test `MatchingExerciseView` pagination and matching on the visible page slice.
  - Callbacks/state to invoke: construct with a `cards` list of >10 entries; click front/back tiles; click "Next"
  - Properties to assert: visible tile count caps at page size (~10); `matched-count` and `exercise-completed` operate on the current page slice; advancing pages resets selection/matched state for the new page
  - Behaviors: matching two tiles on the same page locks them; "Next" loads the following set of cards
  - Covers: Task 6.B.1
  **Depends on 6.B.1.**
- [x] 6.B.2 **[slint-developer + rust-developer]** Generate Flashcards notification — name newly created stacks and switch to the Flashcard tab.
  - [x] 6.B.2.1 **[slint-developer]** Add `generation-notification` and `active-view` properties to `VocabularyAppLogic`; bind `VocabularyPage` tabs to `active-view` and add a dismissible notification banner. — see [6-B-2-1.md](.claude/tasks/subtask/6-B-2-1.md)
  - [x] 6.B.2.2 **[rust-developer]** In `on_generate_exercises_clicked`, diff pre-existing vs. generated stack names, set `generation_notification` naming only the new stacks, and set `active_view = 2`. **Depends on 6.B.2.1.** — see [6-B-2-2.md](.claude/tasks/subtask/6-B-2-2.md)
- [x] 6.B.2.T **[slint-tester]** Test Generate Flashcards notification and tab switch.
  - Callbacks to invoke: `invoke_generate_exercises_clicked()` with a lesson list containing both new and pre-existing stack names
  - Properties to assert: `get_generation_notification()` names only newly created stacks (not pre-existing ones); `get_active_view() == 2` after generation; notification stays empty and `active_view` unchanged when no new stacks are created
  - Behaviors: regenerating with no new lessons produces no notification and no tab switch
  - Covers: Task 6.B.2
  **Depends on 6.B.2.2.**
- [x] 6.B.3 **[slint-developer]** Fix `FlashcardLabel` width overlap: cap width with `min(parent.width, ...)` so it no longer overflows its container border. — see [6-B-3.md](.claude/tasks/subtask/6-B-3.md)

## Phase 6.T: Theme Redesign
**Goal**: Make the color palette swappable via `lib/styles/themes/theme_*.slint` files, each containing the full `Tokens` global, with `styles.slint` re-exporting the active theme via a single line.

> Tasks 6.T1.1 → 6.T1.2 → 6.T1.3 are sequential (each modifies the same file chain). 6.T1.4 is manual verification by task-manager.

- [x] 6.T1.1 **[slint-developer]** Move `Tokens` global from `lib/styles/tokens.slint` into `lib/styles/themes/theme_default.slint` (verbatim contents); delete `tokens.slint`. — see [6-T1-1.md](.claude/tasks/subtask/6-T1-1.md)
- [x] 6.T1.2 **[slint-developer]** Update `lib/styles/styles.slint` to re-export `Tokens` from `themes/theme_default.slint`. (Completed as part of 6.T1.1's commit — import path updated there to keep the build green; no separate commit needed.) **Depends on 6.T1.1.** — see [6-T1-2.md](.claude/tasks/subtask/6-T1-2.md)
- [x] 6.T1.3 **[slint-developer]** Author `lib/styles/themes/theme_solarized_light.slint` with the Solarized Light color mapping (not wired into `styles.slint` — default theme stays active). **Depends on 6.T1.2.** — see [6-T1-3.md](.claude/tasks/subtask/6-T1-3.md)
- [x] 6.T1.4 Manual verification: `cargo run --bin japanese_learn` shows unchanged default palette; `cargo build` green; `cargo clippy --bin japanese_learn` zero warnings. No agent — task-manager runs this directly. **Depends on 6.T1.3.**

## Phase R1: Redesign 1.0
**Goal**: Create `lib/common_component` library; redesign Exercise view with `ExerciseGrid`; rename `LessonStackLabel` → `LessonLabel`; create `TestView`; redesign `ReviewPage` as lesson-centric; redesign `MatchingExerciseView` with submit/result flow.

> R1.1 is independent. R1.1 → R1.2 → R1.3 → R1.4 are strictly sequential (each modifies overlapping Slint import paths).

- [x] R1.1 **[slint-developer]** Create `lib/common_component` libC scaffold; register `@common_component` in root, flashcard, and vocabulary `build.rs` files. — see [R1-1.md](.claude/tasks/subtask/R1-1.md)
- [x] R1.2 **[slint-developer]** Move `CommonBtn` from `lib/flashcard` to `lib/common_component`; update every import site in flashcard, vocabulary, and root UI. **Depends on R1.1.** — see [R1-2.md](.claude/tasks/subtask/R1-2.md)
- [x] R1.3 **[slint-developer]** Move `CommonList` from `lib/flashcard` to `lib/common_component`; update every import site. **Depends on R1.2.** — see [R1-3.md](.claude/tasks/subtask/R1-3.md)
- [x] R1.4 **[slint-developer]** Create `CommonGrid` in `lib/common_component` (4-column grid of configurable `CommonBtn` items). **Depends on R1.3.** — see [R1-4.md](.claude/tasks/subtask/R1-4.md)

> After R1.3: R1.5 and R1.8 may run in parallel (different files). After R1.4: R1.6 may proceed.

- [x] R1.5 **[slint-developer]** Rename `LessonStackLabel` → `LessonLabel` (file rename, component name, all exports and usages). **Depends on R1.3.** — see [R1-5.md](.claude/tasks/subtask/R1-5.md)
- [x] R1.6 **[slint-developer]** Create `ExerciseView` and `ExerciseGrid` in `lib/vocabulary/ui/components/`; export from `vocabulary_lib.slint`. **Depends on R1.4.** — see [R1-6.md](.claude/tasks/subtask/R1-6.md)
- [x] R1.7 **[slint-developer]** Update `VocabularyPage`: use `ExerciseView` + `ExerciseGrid` in Exercise tab; remove Flashcard action-bar tab. **Depends on R1.6.** — see [R1-7.md](.claude/tasks/subtask/R1-7.md)
- [x] R1.8 **[slint-developer]** Redesign `MatchingExerciseView`: 5-row pagination, Submit button, `result-view` flag with correct/incorrect coloring, Close button. **Depends on R1.3.** — see [R1-8.md](.claude/tasks/subtask/R1-8.md)
- [x] R1.9 **[slint-developer]** Create `TestView` in `lib/vocabulary/ui/components/` (CommonGrid with Matching Test → MatchingExerciseView). **Depends on R1.4, R1.8.** — see [R1-9.md](.claude/tasks/subtask/R1-9.md)
- [x] R1.10 **[slint-developer]** Redesign `ReviewPage`: lesson selection list (read-only), `TestView` for selected lesson, heading shows lesson name. **Depends on R1.5, R1.9.** — see [R1-10.md](.claude/tasks/subtask/R1-10.md)

---
## ✅ Milestone 2 Complete — v2.0.0
All phases from 3 through R1 are done. Tagged: `v2.0.0` / `Milestone2`.

---

## Phase 7: Mobile Support (Android + Swipe Gestures)
**Goal**: Add swipe navigation to the study session and prepare the codebase for Android (file-picker gating, touch-target sizing). `cargo apk` build + emulator verification require Android SDK/NDK + `cargo-apk`, none of which are installed in this dev environment — those steps are documented as a pending manual follow-up, not executed here.

> 7.1–7.4 are independent of each other; 7.2 depends on 7.1.

- [x] 7.1 **[slint-developer]** Create `SwipeArea` reusable component in `lib/styles/ui/swipe_area.slint` using `TouchArea` pointer-delta tracking (press position → release position); expose `callback swiped-left()`, `callback swiped-right()`, `callback swiped-down()`, fired when the dominant-axis delta exceeds a threshold. Export from `lib/styles/styles.slint`.
- [x] 7.2 **[slint-developer]** Wire `SwipeArea` into `StudySessionView` (`lib/flashcard/ui/page/study_session_view.slint`) wrapping the flashcard area: swipe-left → Next, swipe-right → Previous, swipe-down → Close, reusing the existing bounds-checked navigation logic. **Depends on 7.1.**
- [x] 7.3 **[rust-developer]** Android-gate `lib/persistent_data/src/file_io.rs`: change the desktop branch from `#[cfg(not(target_arch = "wasm32"))]` to `#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]` (rfd is unsupported on Android per `architecture.md`); add a parallel `#[cfg(target_os = "android")]` no-op stub module (`None`/`false`) mirroring the existing wasm32 stubs.
- [x] 7.4 **[slint-developer]** Touch-target audit: bump `Tokens.size-btn-icon` from `32px` to `44px` in both `lib/styles/themes/theme_default.slint` and `theme_solarized_light.slint`; replace the hardcoded `width: 32px; height: 32px;` icon-button literals in `lib/vocabulary/ui/components/word_form.slint` (tense/example delete buttons) with `Tokens.size-btn-icon`. Verify no layout overflow at the new size.
- [x] 7.5 Document Android build follow-up (no agent — task-manager note, not implemented): `cargo-apk` and Android SDK/NDK are not installed in this environment, so `[package.metadata.android]` Cargo.toml config, the `android_main` entry point, and `cargo apk build`/emulator verification are left as a follow-up for whoever has that tooling available. **Depends on 7.1–7.4.**

## Phase 8: Optimization & Testing
**Goal**: Harden the application — close test gaps, remove a redundant-write inefficiency surfaced by the Phase 8.B fix, and confirm workspace-wide build/lint/test compliance.

> 8.1 and 8.2 are independent. 8.3 depends on both; 8.4 depends on 8.3.

- [x] 8.1 **[rust-developer]** Add Rust unit tests for `update_progress` edge cases in `test/flashcard/tests/study_page.rs`: no stack selected (`selected_stack_index == -1` → counts reset to 0), and a stack with zero cards (`total_count == 0`).
- [x] 8.2 **[rust-developer]** Optimize `on_known_changed` in `lib/flashcard/src/lib.rs`: skip the `set_flashcard_list` rebuild and `save_stacks()` call when the incoming `known` value already matches the card's current value (no-op write avoidance — relevant after the Phase 8.B resync fix, which now fires `known-changed` on every card navigation, not just on an actual toggle). Add a unit test confirming `known` and counts stay correct across repeated identical calls.
- [x] 8.3 Run `cargo build --workspace`, `cargo clippy --workspace`, and `cargo test --workspace -- --test-threads=1` (no agent — task-manager verification); confirm zero errors/warnings and all tests green. **Depends on 8.1, 8.2.**
- [x] 8.4 Write `docs/testing-report.md` summarizing test counts per crate and the 8.2 optimization (no agent). **Depends on 8.3.**

## Phase 8.B: Bug Fixes (pre-Analytics)
**Goal**: Fix known/unknown count accuracy before Analytics depends on it.

- [x] 8.B.1 **[slint-developer]** Fix `Flashcard.known` reactivity in `lib/flashcard/ui/components/flashcard.slint`: the `in-out property <bool> known: data.known` default binding stops tracking `data.known` after `CommonBtn.checked <=> known` performs its first write (Slint severs a default binding on first external write to an in-out property). Add `changed data => { known = data.known; }` so `known` always resyncs to the current card whenever `data` changes (including on Prev/Next navigation), so `known-changed` always reports the correct card/value and `known-count`/`total-count` stay accurate after navigation.
- [x] 8.B.2 Manual verification on Windows (no agent): open a stack, toggle a card's known status, navigate Prev/Next through several cards, confirm the known icon and known-count reflect each card's actual persisted status (not the previously toggled card's value). **Depends on 8.B.1.**

## Phase 8.V: Extended Vocabulary Data Model
**Goal**: Each `tense:` entry becomes its own flashcard; each `example:` sentence becomes its own sentence flashcard, persisted separately to `sentences.json`. Per `extended-vocab.md`. The tense-parsing fix (8.V.1) is a live data-loss bug fix — every bundled `.md` file already uses `tense: <label> : <conjugation>` but the parser splits on `'|'`, so all tense data is silently dropped today.

> 8.V.1, 8.V.2, 8.V.3, 8.V.6, 8.V.8 are independent of each other and may start immediately.

- [x] 8.V.1 **[rust-developer]** Fix the live data-loss bug in `lib/vocabulary/src/vocabulary_markdown_io.rs`: parse `tense: <label> : <conjugation>` (split the field value on the **second** colon, not `'|'`); a tense with no conjugation after the second colon produces no `TenseData` entry. Update the existing `parse_word_with_tenses_and_examples` test to the colon format and add a test reproducing the literal bundled-data pattern (`tense: present-formal : たべます`, `tense: negative-present-formal:` with nothing after → dropped).
- [x] 8.V.2 **[rust-developer]** Add `TenseType` enum to `lib/exercise_generator` (new file `src/tense_type.rs`): fixed N5 variants (e.g. `Present`, `PastFormal`, `PastInformal`, `NegativePresentFormal`, `NegativePresentInformal`, `NegativePastFormal`, `NegativePastInformal`, `PotentialFormFormal`, `PotentialFormInformal`, `PotentialNegativeForm`, `PotentialNegativeFormInformal`, `TeForm`) plus `Other(String)` fallback; `from_label(label: &str) -> TenseType` strips parenthetical qualifiers (e.g. `"(nai-form)"`) before matching and falls back to `Other(original_label)` for unrecognized input; `display_label(&self) -> String` returns the canonical kebab-case label. Re-export from `lib.rs`. Unit tests for matching, parenthetical stripping, and the `Other` fallback.
- [x] 8.V.3 **[rust-developer]** Add `ExampleEntry { sentence: String, meaning: String }` to `lib/exercise_generator/src/models.rs`; change `VocabularyWord.examples` from `Vec<String>` to `Vec<ExampleEntry>`; update the `flashcard_transformer.rs` test fixture (`word()` helper) accordingly. Re-export `ExampleEntry` from `lib.rs`.
- [x] 8.V.4 **[rust-developer]** Extend `FlashcardExerciseTransformer::make_cards` (same transformer, evolved mapping): for each of the word's `tenses` with a non-empty `conjugation`, emit one additional `FlashcardCardData` — `front: conjugation`, `back: format!("{} — {}", word.meaning, derived_type)` where `derived_type` combines `word.word_type` and `TenseType::from_label(&tense.name).display_label()`, `known: false`, `is_kanji: false`. Update/extend unit tests to cover a word with tenses. **Depends on 8.V.2, 8.V.3.**
- [x] 8.V.5 **[rust-developer]** Add `SentenceExerciseTransformer` (`src/sentence_transformer.rs`): `VocabularyLesson → FlashcardStackData`, one `FlashcardCardData` per `ExampleEntry` (`front: sentence`, `back: meaning`, `known: false`, `is_kanji: false`); lessons with no examples produce no stack entry for that word. Add `ExerciseRequest::Sentence` / `ExerciseOutput::Sentence(Vec<FlashcardStackData>)` to `transformer.rs` and the matching dispatch arm in `service.rs`. Unit tests mirroring `flashcard_transformer.rs`'s coverage. **Depends on 8.V.3.**
- [x] 8.V.6 **[rust-developer]** Restructure vocabulary examples to carry a sentence **and** a meaning (touches both the Slint struct and its Rust handler in the same crate, so this is one atomic commit):
  - `lib/vocabulary/ui/vocabulary_app_logic.slint`: add `export struct ExampleData { sentence: string, meaning: string }`; change `VocabularyWordModel.examples` from `[string]` to `[ExampleData]`; change `callback word-example-add-confirmed(lesson-idx: int, word-idx: int, example: string)` to `(lesson-idx: int, word-idx: int, sentence: string, meaning: string)`.
  - `lib/vocabulary/ui/components/word_form.slint`: `in property <[string]> example-data` → `in property <[ExampleData]> example-data`; display both `example.sentence` and `example.meaning` per row; add-example form gains a second "Meaning" `TextInput`; update the confirm button to call `word-example-add-confirmed(root.lesson-idx, root.word-idx, sentence-input.text, meaning-input.text)`.
  - `lib/vocabulary/src/lib.rs`: add `ExampleData { sentence: String, meaning: String }` shadow struct (mirroring `TenseData`); change `WordData.examples` to `Vec<ExampleData>`; update `slint_to_lessons`/`lessons_to_slint` conversions; update `on_word_example_add_confirmed` to the new two-string signature; update `on_generate_exercises_clicked`'s `VocabularyWord.examples` mapping to build `exercise_generator::ExampleEntry { sentence, meaning }`.
  - Build the whole workspace (`cargo build --bin japanese_learn`) before committing — this crate's Slint and Rust sides compile together.
- [x] 8.V.7 **[rust-developer]** Rewrite example parsing in `lib/vocabulary/src/vocabulary_markdown_io.rs` to the `example: <sentence> : <meaning>` format (split on the first colon after the `example:` key, same second-colon pattern as tense), producing `ExampleData { sentence, meaning }` instead of a plain string; drop an example whose sentence is empty after trimming. Update existing example-related tests and add one reproducing the literal bundled-data pattern. **Depends on 8.V.6.**
- [x] 8.V.8 **[slint-developer]** Add `in-out property <[FlashcardStackModel]> sentence-stack-list: []` and `in-out property <int> selected-sentence-stack-index: -1` to `FlashcardAppLogic` (`lib/flashcard/ui/flashcard_app_logic.slint`). Declaration only.
- [x] 8.V.9 **[slint-developer]** `FlashcardManagerView` (`lib/flashcard/ui/page/flashcard_manager_view.slint`): add a "Flashcards"/"Sentences" toggle (two `CommonBtn`s) controlling which list/detail is shown; the Sentences view reuses `CommonList`/`FlashcardList`/`FlashcardStack` bound to `sentence-stack-list`/`selected-sentence-stack-index` (study-clicked and close-clicked wired the same way; the mutation callbacks — add/edit/delete/reorder card, delete stack — are intentionally left unwired for the Sentences view, since sentence stacks are machine-generated and fully replaced on every "Generate Exercises" run). **Depends on 8.V.8.**
- [x] 8.V.10 **[rust-developer]** Add `sentences.json` persistence to `lib/flashcard/src/lib.rs`: `SENTENCES_FILE` const, `save_sentences`/`load_sentences` mirroring `save_stacks`/`load_stacks`, and `pub fn save_sentence_list<T>(ui: &T)` mirroring `save_current_list`. Call `load_sentences()` at `init()` startup to populate `sentence_stack_list`. **Depends on 8.V.8.**
- [x] 8.V.11 **[rust-developer]** In `lib/vocabulary/src/lib.rs`'s `on_generate_exercises_clicked`, additionally dispatch `ExerciseRequest::Sentence` against the same lesson list, convert the `ExerciseOutput::Sentence` stacks to `flashcard::flashcard::FlashcardStackModel` (same conversion shape as the Flashcard branch), call `flashcard_logic.set_sentence_stack_list(...)` and `flashcard::save_sentence_list(&ui)`. See the cross-library architecture plan `.claude/tasks/architecture/task-8-V-11.md` (authored by project-owner) before implementing. **Depends on 8.V.5, 8.V.6, 8.V.7, 8.V.10.**
- [x] 8.V.12 Update bundled default `.md` (and matching `.json`) datasets in `lib/vocabulary/ui/data/` so every `example:` line carries a meaning in the `sentence : meaning` format (no agent — data authored directly by task-manager). **Depends on 8.V.7.**
- [x] 8.V.13 **[slint-tester]** Test tense-derived and sentence flashcard generation end-to-end in `test/vocabulary/`: a lesson with a word that has tenses and examples → `invoke_generate_exercises_clicked()` → assert the resulting flashcard stack contains the base card plus one card per tense with a conjugation, and assert `sentence-stack-list` contains one card per example with `front == sentence`, `back == meaning`. **Depends on 8.V.11.**
- [x] 8.V.14 Manual verification on Windows (no agent): author a word with two tenses and one example, click Generate Exercises, confirm `data/stacks.json` gains the tense-derived cards and `data/sentences.json` is created, and confirm the FlashcardManagerView Sentences toggle shows the new sentence stack. **Depends on 8.V.13.**

## Backlog (Unscheduled)
**Goal**: Ideas not yet assigned to a phase. Renumbered from the former "Phase 7: Future Backlog" — superseded by the real Phase 7 (Mobile Support) above; items already covered by a scheduled phase are noted.

- [ ] BL.1 Add audio playback (Japanese text-to-speech integration). Superseded by Phase 11 (Audio) in `plan.md`.
- [ ] BL.2 Implement spaced repetition algorithms for study scheduling.
- [ ] BL.3 Add synchronization across devices (future cloud sync).
- [ ] BL.4 Add WASM-compatible import/export: replace `rfd` + `std::fs` with a browser file-input element via `web-sys` and JavaScript interop. Gate with `#[cfg(target_arch = "wasm32")]`.
- [ ] BL.5 Add analytics and reporting features (progress charts, study statistics). Superseded by Phase 9 (Analytics) in `plan.md`.

# Deliverables
- Each task produces incremental functionality aligned with the plan.
- Completion of all tasks results in:
  - A working Windows + WebAssembly application with flashcard management, study mode, and markdown-based import/export.
  - Documentation of architecture, modules, and usage.
  - Rust test suite covering core features.
  - Backlog items prepared for future iterations.

