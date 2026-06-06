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

**Milestone 1 should include the following features**

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

## Persistent Data Management (Desktop - Milestone 1)
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

**Urgent requirements change — UI redesign (post Phase 6)**

### StudyPage tab restructure
Remove the "Flashcard" tab from `StudyPage`'s topic tab row. The row must contain exactly
three tabs: **Vocabulary** (index 0), **Grammar** (index 1), **Reading** (index 2).

### VocabularyPage redesign
`VocabularyPage` gains an action bar with four controls: three tab-style `CommonBtn`s
(**Lesson**, **Exercise**, **Flashcard**) that switch the visible view, and one direct-action
`CommonBtn` (**Import Lesson**) that fires `VocabularyAppLogic.import-vocabulary-clicked()`
without changing the active view.

#### Lesson view (default, active-view = 0)
- Render a scrollable list of lessons as `LessonStackLabel` items inside `LessonStackList`,
  following the same layout and interaction pattern as `FlashcardLabel` + `FlashcardList`.
- `LessonStackLabel` inherits `CommonBtn`; its text is the lesson name.
- `LessonStackList` follows the **Vertically stacked pattern**: lesson labels in a `Flickable`,
  an inline create-lesson form that slides up, and an "＋ Add Lesson" button.
- Clicking a `LessonStackLabel` sets `VocabularyAppLogic.selected-lesson-index` and reveals
  `LessonDetailView` (full lesson content + word list via existing `LessonDetailPane`, plus a
  close `CommonBtn` that resets the index to -1). Uses the Vertically stacked pattern.
- The existing `LessonList` component is superseded by `LessonStackList` in this view.

#### Exercise view (active-view = 1)
- Shows **Generate Flashcards** and **Export Vocabulary** action buttons.
- No structural change to existing exercise generation logic.

#### Flashcard view (active-view = 2)
- Shows `FlashcardManagerView`: a new component that encapsulates the full flashcard
  management UI previously under `StudyPage`'s Flashcard tab (background dismiss `TouchArea`,
  Import/Export header, create-stack inline form, `FlashcardStack` detail pane,
  `CommonList` + `FlashcardList`).
- `StudySessionView` must first be moved to `lib/flashcard/ui/components/` and exported via
  `flashcard_lib.slint` so that `FlashcardManagerView` (which lives in `lib/vocabulary`) can
  import it from `@flashcard` without creating a circular dependency.

#### New components
| Component | File | Library |
|---|---|---|
| `StudySessionView` (moved) | `lib/flashcard/ui/components/study_session_view.slint` | flashcard |
| `LessonStackLabel` | `lib/vocabulary/ui/components/lesson_stack_label.slint` | vocabulary |
| `LessonStackList` | `lib/vocabulary/ui/components/lesson_stack_list.slint` | vocabulary |
| `LessonDetailView` | `lib/vocabulary/ui/components/lesson_detail_view.slint` | vocabulary |
| `FlashcardManagerView` | `lib/vocabulary/ui/components/flashcard_manager_view.slint` | vocabulary |

**User flow**: StudyPage → Vocabulary tab → Import Lesson (action) → imports markdown →
Lesson tab → view/add/delete lessons via `LessonStackList` → click lesson label →
`LessonDetailView` (view/add/delete words) → Close. Or: Flashcard tab →
`FlashcardManagerView` → add/delete/edit stacks → click stack label → `FlashcardStack`
detail → view/add/delete/edit flashcards.

**Milestone 2 should include the following features**

## Vocabulary Study Mode and Flashcard Exercise Generation

### Navigation
- `StudyPage` gains a topic selector. Available topics: `Vocabulary`, `Grammar` (placeholder), `Reading` (placeholder).
- Selecting `Vocabulary` shows two sub-views: `Lesson` (create and edit vocabulary lessons) and `Exercises` (view and study generated flashcard exercises).

### Vocabulary Lesson Input
- Users **must** be able to create, name, and delete vocabulary lessons.
- For each lesson, users **must** be able to add, edit, and delete words. Each word has:
  - `spelling` (required): hiragana, katakana, or romaji pronunciation of the word.
  - `kanji` (optional): kanji representation of the same word.
  - `explanation`:
    - `meaning` (required): Vietnamese meaning.
    - `type` (optional): part of speech — noun, verb, adjective, adverb, particle, pronoun, conjunction, interjection, or adjectival noun. User types or selects the value.
    - `tense` (optional, repeatable): each entry is a label (e.g., "positive-polite") paired with the conjugated form (e.g., "食べます"). Users may add as many entries as needed.
    - `example` (optional, repeatable): one or more Japanese sentences illustrating usage.

### Flashcard Exercise Generation
- The application **must** generate a `Flashcard` exercise from vocabulary lessons on demand.
- Each vocabulary lesson maps to one flashcard stack; the stack name equals the lesson name.
- For each word in a lesson:
  - If `kanji` is provided: generate **two** flashcards — one with `kanji` on the front and one with `spelling` on the front. Both share the same `explanation` on the back.
  - If `kanji` is not provided: generate **one** flashcard with `spelling` on the front and `explanation` on the back.
- The back face of every generated flashcard **must** include: `meaning`, `type` (if set), all `tense` entries (if any), and all `example` sentences (if any).
- Generated flashcard stacks are studied and managed using the existing `## Flashcard Management` and `## Study Mode` features.

### Data Separation and Re-creation
- Vocabulary lesson data **must** be stored independently from flashcard exercise data (separate persistence files).
- Users **may** freely edit generated flashcards (add, delete, reorder) without affecting the source vocabulary lessons.
- A **"Re-create Flashcards"** action **must** be available. It deletes all generated flashcard stacks and regenerates them from current vocabulary lesson data. Users **must** be prompted to confirm before the destructive re-creation.

### Internal Data Stores
- A **word bank** aggregates all unique words across all vocabulary lessons. It is an internal store used by exercise generation and future exercise types — not a user-navigable screen.
- A **sentence bank** aggregates all example sentences across all vocabulary lessons. It is an internal store used by future exercise types (e.g., listening, reconstruction).

## Persistent Data Management (Desktop - Milestone 2)
- The application **must** support import of vocabulary lessons from a markdown file.
- The vocabulary markdown format uses `## Lesson Name` headings to delimit lessons. Under each heading, word entries follow this structure:

  ```
  ### たべる
  kanji: 食べる
  meaning: to eat
  type: verb
  tense: positive-polite → 食べます
  tense: negative-polite → 食べません
  example: 私は毎日ご飯を食べる。
  example: 彼は魚を食べません。
  ```

  The `### <spelling>` subheading starts each word entry. `kanji:`, `type:`, `tense:`, and `example:` are optional. `tense:` and `example:` may repeat for multiple values.
- The application **must** support export of all vocabulary lessons to a markdown file in the same format.
- File dialogs **must** use `rfd`; all `std::fs` calls **must** be gated `#[cfg(not(target_arch = "wasm32"))]`.

## Vocabulary Review Mode
- When the user opens the Review Page, it displays the same flashcard stack list as Study Mode.
- Users **cannot** manage stacks or cards from the Review Page (no add, delete, or reorder).
- When a user selects a stack, the application presents a **pair-matching exercise**:
  - All flashcard fronts and all flashcard backs from the selected stack are displayed as separate, independently shuffled tiles.
  - Users must tap or drag to pair each front tile with its correct back tile.
  - When all pairs are correctly matched, the exercise is marked as passed.

## Style design
- The application should have the following day mode colors: 
  - #CCD5AE , #E9EDC9 , #FEFAE0, #FAEDCD, #D4A373 ,  #251504 
  - Darker colors should be used for border, text. 
  - Easy to view for studying.
  - The tab buttons "Vocabulary", "Grammar", "Reading" should be distinguishable from lower buttons "Vocabulary" page such as "Lesson", "Exercise", "Flashcard", "Import Lesson".
  - If the colors are not enough, use your best judgement to provide suitable collors for this pallete. Use this page to generate good colors if possible https://coolors.co/palettes/trending 
- Night mode colors are not yet designed, but will be offered.


## Button Arrangements and Colors:
- Buttons on the same tab should have multual exclusive behavior: since only one tab is open, only one should change color on click. 
- Buttons change color when user click and hold.
- Tab buttons that switch page view changes colors when user click and hold. 
- Buttons to open containers, smaller windows, or to load a feature (import file) doesn't have color.
- Disable buttons should not appear on layout. 
- Flashcard must have "flipped" effect. Keep the effect implementation in "animation" for future reuse.
- Flashcard whose front is "kanji" should have extra large size. Use nice font to represent brush stroke. 
- User can resize window; pages and views should be responsive. 
- "Add Lesson" is too wide. Keep an appropriate maximum width and same height for buttons.
- There is a bug, when "Add Lession" is clicked, the "Confirm" and "Cancel" button overlaps with another container. Fix it!

**Milestone 3 should include the following development**
- Milestone 3 delivers **Analytics** and **Grammar Study Mode** as defined in the Future Backlog below.

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
