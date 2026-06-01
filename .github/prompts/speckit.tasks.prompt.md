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
- [ ] 1.2 Configure build targets for Windows and WebAssembly. Configure Cargo.toml to provide alias for both targets so that the build commands can be easily executed from the terminal. The alias should be named `build-windows` for Windows and `build-web` for WebAssembly, and should execute the appropriate build command for each target. Update Readme.md to include instructions on how to use these aliases for building the application on both platforms.
```toml
[alias]
build-windows = "build --target x86_64-pc-windows-msvc"
build-web = "build --target wasm32-unknown-unknown"
```
- [ ] 1.3 Create base UI layout (main window, navigation).
- [ ] 1.4 Implement sample data models with hardcoded flashcards.
- [ ] 1.5 Verify compilation and run on Windows and WebAssembly.

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
