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
- Users **must** be able to change a flashcard's status between "known" and "unknown."
- A progress indicator **must** display known-count vs total-count within the active session.

## Persistent Data Management (Desktop)
- The application **must** support import of flashcard stacks from a markdown file.  
  Format: `## Stack Name` headings delimit stacks; a GFM pipe table (`| Japanese | Meaning |`) under each heading lists cards.
- The application **must** support export of all stacks to a markdown file in the same format.
- File dialogs **must** use `rfd`; all `std::fs` calls **must** be gated `#[cfg(not(target_arch = "wasm32"))]`.

## Automation Tests
- Automated tests **must** be written and kept as a passing baseline.
- All builds **must** pass baseline tests before being committed.
- Baseline tests **must** remain unchanged unless a design change requires it; any such change **must** be justified.

## Universal Styling Library
- A dedicated `lib/styles` library (libC pattern — Slint-only, no Rust backend) **must** be created to own all design tokens.
- The library **must** export: color palette, typography scale, spacing constants, border tokens, and animation-curve constants.
- All other libraries and pages **must** import tokens from `@styles`; hardcoded colors, sizes, or durations **must not** appear in component files.

# Requirements (Later / Future Backlog)

## Analytics
- The application **should** track study sessions: date, stack studied, total cards, known count.
- The application **should** display progress analytics per stack: known vs unknown bar, cards studied over time.
- Charts **must** be implemented in pure Slint using proportional `Rectangle` widths/heights — no charting library.

## Grammar Study Mode
- Users **should** be able to create grammar lessons.  
  Each lesson has: a **title**, a **structure description** (free text, e.g., "Verb て + います"), and a list of **sample sentences** each with a Japanese string and a Vietnamese meaning.
- Each sentence **may** include a `tokens` array (pre-split word chips) to enable the reconstruction exercise; if absent, that exercise is skipped for the sentence.
- The application **should** provide three exercise types per lesson:
  1. **Matching** — shuffled sentence blocks paired with shuffled meaning blocks; user drags to match.
  2. **Reconstruction** — shuffled word-chip blocks from `tokens`; user drags to reconstruct the sentence in order.
  3. **Fill-in-the-blank** — one token randomly removed; user types the missing word into a `TextInput`.
- For each exercise, the user **may** mark it as "passed."  
  When all exercises in a lesson are passed, the lesson is marked "known."
- Grammar exercises **must** be implemented using Slint pointer/touch events and standard Rust string logic; MeCab or other NLP crates **must not** be used.

## Audio — TTS
- The application **should** play Japanese TTS audio for words and sentences.
- Desktop: use the `tts` crate (wraps OS voice engine — Windows SAPI, macOS AVSpeechSynthesizer).  
  Users must have a Japanese TTS voice installed at the OS level.
- WebAssembly: use the browser's Web Speech API via `web_sys` — no `tts` crate on this target.
- The audio service **must** be implemented in `lib/audio` (libB pattern); Slint components fire a callback; the Rust handler calls the TTS engine.
- A "Listen" button **must** appear on each flashcard in study mode and on each sentence in grammar study mode.

## Listening Study Mode and Review Mode
- **Study mode**: a "Listen" button plays TTS for the currently shown word or sentence (depends on Audio).
- **Review mode — Multiple Choice**: the application reads a randomly selected word or sentence aloud; the user selects the matching text from 3–4 choices displayed on screen.
- Audio must be triggered from the Slint UI via a callback; no audio logic in `.slint` files.

## Mobile Support (Android)
- The application **should** be deployable to Android using Slint's built-in Android backend.
- Build tool: `cargo-apk` (dev toolchain, not a crate dependency).
- File dialogs (`rfd`) are not supported on Android; a platform-specific file-picker integration is required, gated with `#[cfg(target_os = "android")]`.
- Swipe gestures **should** be supported: swipe left/right to navigate cards in study mode; swipe down to close a stack.
- Swipe **must** be implemented using Slint's `TouchArea` pointer delta events — no external gesture library.
- All UI components **must** remain responsive to touch-sized targets (minimum 44 dp touch area).

## Cloud Synchronization
- The application **may** support synchronization of flashcard stacks across devices.
- Implementation approach is deferred; it must not introduce breaking changes to the local data format.
