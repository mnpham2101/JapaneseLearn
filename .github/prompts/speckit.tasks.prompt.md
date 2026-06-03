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
- [ ] 1.4.3 Modify `FlashcardStack` in `ui/components/flashcard_stack.slint` to display 2 plain text boxes per card (`jap-obj` and `explanation` as two `Text` elements, no flip or toggle behaviour). In `ui/pages/study_page.slint`, replace the placeholder `Rectangle` detail pane with the actual `FlashcardStack` component bound to `flashcardList[selected-stack-index]`; wire `close-clicked` to reset `selected-stack-index = -1`. The existing 3 hardcoded stacks are sufficient to verify multi-stack behaviour — task 1.4.4 is folded into this task.

## Phase 2: Flashcard Management
- [ ] 2.1 Implement stack creation with naming functionality.
- [ ] 2.2 Implement add flashcard (Japanese word + Vietnamese meaning).
- [ ] 2.3 Implement edit flashcard functionality.
- [ ] 2.4 Implement delete flashcard functionality.
- [ ] 2.5 Implement list view to display flashcards in a stack.
- [ ] 2.6 Implement drag‑and‑drop reordering of flashcards in list view.
- [ ] 2.7 Implement persistence of flashcard data using local JSON/Markdown files.
- [ ] 2.8 Test flashcard CRUD operations manually on Windows and WebAssembly.

## Phase 3: Study Mode
- [ ] 3.1 Implement study mode view with single‑card presentation.
- [ ] 3.2 Implement reveal mechanism (Japanese word first, Vietnamese meaning hidden).
- [ ] 3.3 Implement marking of flashcards as “known” or “unknown.”
- [ ] 3.4 Implement ability to toggle flashcard status between “known” and “unknown.”
- [ ] 3.5 Track user progress within each stack (known vs unknown counts).
- [ ] 3.6 Test study mode interactions manually on Windows and WebAssembly.

## Phase 4: Optimization & Testing
- [ ] 4.1 Optimize rendering performance for Windows and WebAssembly targets.
- [ ] 4.2 Test UI responsiveness across both targets.
- [ ] 4.3 Add Rust unit tests for core logic (flashcard CRUD, study mode state).
- [ ] 4.4 Add Rust integration tests for data persistence.
- [ ] 4.5 Ensure compliance with constitution best practices (UI separation, modularity).
- [ ] 4.6 Document testing results and performance benchmarks.

## Phase 5: Future Backlog (Extensible)
- [ ] 5.1 Add audio playback (Japanese text‑to‑speech integration).
- [ ] 5.2 Implement spaced repetition algorithms for study scheduling.
- [ ] 5.3 Add synchronization across devices (future cloud sync).
- [ ] 5.4 Implement export/import of stacks via markdown files.
- [ ] 5.5 Add analytics and reporting features (progress charts, study statistics).

# Deliverables
- Each task produces incremental functionality aligned with the plan.
- Completion of all tasks results in:
  - A working Windows + WebAssembly application with flashcard management and study mode.
  - Documentation of architecture, modules, and usage.
  - Rust test suite covering core features.
  - Backlog items prepared for future iterations.
