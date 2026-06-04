---
agent: speckit.specify
---
/speckit.specify

# Application Purpose
The application supports Japanese language learning by providing interactive flashcards.  
Learners can input Japanese words with Vietnamese meanings, organize them into stacks, and practice through study sessions.  
The goal is to make learning efficient, engaging, and trackable across desktop and WebAssembly platforms.

# Core Objectives
- Provide simple flashcard creation and management.
- Enable structured study sessions with progress tracking.
- Ensure lightweight performance and maintainability.
- Follow best practices defined in the constitution for UI, Rust, and general software development.

# Requirements (Now)
## Flashcard Management
- Users **must** be able to create stacks of flashcards with names.
- Users **must** be able to add Japanese words with Vietnamese meanings.
- Flashcards **must** be displayed in a list view.
- Users **must** be able to edit or delete flashcards.
- Users **must** be able to reorder flashcards by drag‑and‑drop.

## Study Mode
- The application **must** provide a study mode to review flashcards.
- Flashcards **must** be presented one at a time.
- The Japanese word **must** be shown first, with the Vietnamese meaning hidden until revealed.
- Users **must** be able to mark flashcards as "known" or "unknown."
- Users **must** be able to change a flashcard’s status between "known" and "unknown."

## Persistent data management
- Add export/import of stacks via markdown files. User should be able to provide markdown file with structured format; the application can read the file and generate the Flashcards.

# Requirements (Later / Future Backlog)
## Data Management
- The application **may** support export/import of flashcard stacks via text or markdown files.
- The application **should** allow synchronization across devices.

## Analytics
- The application **should** provide user progress analytics and reporting.
- The application **may** visualize study statistics (e.g., charts of known vs unknown).

