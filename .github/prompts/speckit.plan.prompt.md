---
agent: speckit.plan
---
/speckit.plan

# Application Plan
This plan outlines the steps to build a cross‑platform Japanese language learning application.  
The plan is based on the requirements defined in `speckit.specify`, the constitution defined in `speckit.constitution`, and incorporates explicit tech stack and architecture choices.

# Goals
- Deliver a functional flashcard system for Japanese–Vietnamese learning.
- Provide a study mode with progress tracking.
- Ensure performance, maintainability, and cross‑platform compatibility.
- Follow best practices defined in the constitution for UI, Rust, and general programming.

# Tech Stack
- **Programming Language:** Rust (for core logic, safety, performance).
- **UI Framework:** Slint (for declarative, lightweight cross‑platform UI).
- **Cross‑Platform Targets:** Desktop (Windows) and WebAssembly (browser).
- **Data Storage:** Local file storage (JSON/Markdown) for persistence; extensible to cloud sync later.
- **Testing:** Rust unit tests + integration tests for logic; manual testing for UI interactions across Windows and WebAssembly.
- **Version Control & CI/CD:** GitHub repository with continuous integration for builds and tests.

# Architecture Choices
- **Layered Architecture:**
  - **UI Layer (Slint):** Handles presentation, user interactions, and property bindings.
  - **Application Layer (Rust):** Implements business logic (flashcard management, study mode).
  - **Data Layer (Rust):** Manages persistence (local storage, import/export).
- **Separation of Concerns:** UI logic, business logic, and data access are strictly separated into modules.
- **Extensibility:** Future features (audio playback, spaced repetition, analytics, synchronization) can be added without disrupting core modules.
- **Cross‑Platform Strategy:** Shared Rust core compiled for Windows and WebAssembly; Slint UI adapted per target.

# Phases

## Phase 1: Foundation
- Initialize Rust project and Slint UI structure.
- Configure build for Windows and WebAssembly targets.
- Establish base UI layout and navigation flow.
- Implement sample data models with hardcoded flashcards.

## Phase 2: Flashcard Management
- Implement stack creation with naming.
- Implement add/edit/delete functionality for flashcards.
- Implement list view to display flashcards.
- Implement drag‑and‑drop reordering of flashcards.
- Implement persistence of flashcard data (local JSON/Markdown).

## Phase 3: Study Mode
- Implement study mode view with single‑card presentation.
- Implement reveal mechanism (Japanese word first, Vietnamese meaning hidden).
- Implement marking of flashcards as “known” or “unknown.”
- Implement ability to toggle status between “known” and “unknown.”
- Track user progress within each stack.

## Phase 4: Optimization & Testing
- Optimize rendering performance across Windows and WebAssembly.
- Test UI responsiveness across both targets.
- Add automated tests for Rust core logic.
- Ensure compliance with constitution best practices.

## Phase 5: Future Backlog (Extensible)
- Add audio playback (Japanese text‑to‑speech).
- Add spaced repetition algorithms.
- Add synchronization across devices.
- Add export/import of stacks via markdown files.
- Add analytics and reporting features.

# Deliverables
- Each phase delivers a working increment of the application with defined features.
- Final deliverable includes:
  - A working cross‑platform application with flashcard management and study mode.
  - Documentation of architecture, modules, and usage.
  - Test suite covering Rust core features.
  - Backlog items documented for future iterations.