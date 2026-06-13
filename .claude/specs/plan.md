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

| Concern              | Choice                                        | Rationale                                          |
| -------------------- | --------------------------------------------- | -------------------------------------------------- |
| Language             | Rust                                          | Safety, performance, cross-platform                |
| UI framework         | Slint                                         | Declarative, lightweight, desktop + WASM + Android |
| Data storage         | JSON (serde_json) + Markdown (pulldown-cmark) | Already in use; no extra dep                       |
| File dialogs         | rfd                                           | Minimal, async-optional, desktop-only              |
| Desktop TTS          | tts 0.26                                      | Wraps OS voice engine; zero binary overhead        |
| WASM TTS             | web_sys (SpeechSynthesis)                     | Browser-native; already in wasm-bindgen ecosystem  |
| Charts               | Pure Slint (Rectangle bindings)               | No charting library; zero extra dep                |
| Grammar tokenization | User-supplied token arrays                    | Avoids MeCab/Kuromoji; no NLP library              |
| Android              | Slint Android backend + cargo-apk             | Built-in; no extra crate                           |
| Swipe gestures       | Slint TouchArea delta events                  | Built-in; no gesture library                       |

> **Dependency rule**: only `tts` is added beyond the current set (`slint`, `serde`, `serde_json`, `pulldown-cmark`, `rfd`). All other features are implemented with built-in Slint primitives or standard Rust.

# Architecture

## Library Types
See `.claude/rules/architecture.md` for the full libA / libB / libC definitions.

## Library Catalogue

| Crate                    | Type                             | Phase | Responsibility                                                                                                                                          |
| ------------------------ | -------------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `lib/flashcard`          | libA                             | 1–4   | Flashcard CRUD, stack management, session persistence, study mode                                                                                       |
| `lib/styles`             | libC (Slint folder, not a crate) | 3     | Design tokens, color palette, typography, spacing, animation curves                                                                                     |
| `lib/persistent_data`    | libB                             | 5     | Basic flashcard markdown + JSON file I/O, file dialogs                                                                                                  |
| `lib/vocabulary`         | libA                             | 6     | Vocabulary lesson CRUD UI, lesson persistence to `vocabulary.json`, vocabulary markdown import/export; word/sentence banks are internal data structures |
| `lib/exercise_generator` | libD                             | 6     | Transformer service: converts `VocabularyLesson` data into exercise datasets on demand; no Slint, no `build.rs`, no `init()`                            |
| `lib/analytics`          | libA                             | 9     | Session logging, statistics computation, chart Slint components                                                                                         |
| `lib/grammar`            | libA                             | 10    | Grammar lesson models, exercise UI, exercise engine                                                                                                     |
| `lib/audio`              | libB                             | 11    | TTS and playback — OS engine on desktop, Web Speech API on WASM                                                                                         |

## Layered Architecture
- **UI Layer (Slint)**: presentation, property bindings, user interactions.
- **Application Layer (Rust `lib/*`)**: business logic, callback wiring, model manipulation.
- **Data Layer (Rust `lib/persistent_data`, `lib/audio`)**: file I/O, platform services.
- All callback wiring happens in each library's `init()` — never in `src/main.rs`.

# Phases

## Phase 1: Foundation ✓
- Initialize Rust project and Slint UI structure.
- Configure build for Windows and WebAssembly targets.
- Establish base UI layout and navigation flow.
- Implement sample data models and flashcard components.
- Create `lib/flashcard` scaffold (libA pattern).

## Phase 2: Flashcard Management ✓
- Implement stack creation, add/edit/delete, drag‑to‑reorder.
- Implement JSON session persistence.
- Write automation CRUD tests.

## Phase 3: Universal Styling Library ✓
**Goal**: Extract all design tokens into a standalone `lib/styles` folder (libC) so every component in every library imports from `@styles` — no hardcoded colors, sizes, or durations anywhere.

- **Library type**: libC — pure Slint folder, **not a Rust crate** (no `Cargo.toml`, no `src/`).
- `lib/styles/themes/`: one `theme_*.slint` file per color theme, each exporting an identical `Tokens` interface (e.g. `theme_default.slint`, `theme_solarized_light.slint`) — see `slint-code-style.md` § *Swappable theme files*. The active theme is selected by a single re-export line in `styles.slint`; switching themes is a one-line search-and-replace.
- `lib/styles/animations.slint`: easing-curve constants and duration tokens.
- `lib/styles/styles.slint`: entry file — re-exports the active theme's `Tokens` and `Animations`.
- Each client library and the root app registers the path in its `build.rs` via `with_library_paths`; see `slint-code-style.md` § *Use purely Slint reusable library*.
- Migrate all `.slint` files in `lib/flashcard/` and root `ui/` to import from `@styles`.

## Phase 4: Study Mode ✓
**Goal**: Deliver a functional single-card study session within the existing `StudyPage`, wired to live Rust-computed progress counts. Depends on Phase 3 (`@styles` tokens must be available).

- Add a study session view to `StudyPage`: one `Flashcard` at a time with Previous/Next navigation and a close callback.
- Wire the tap-to-flip reveal mechanism (`Flashcard.show-back`).
- Wire the known/unknown toggle (`checkable: true; checked <=> known`).
- Add a progress indicator showing known-count / total-count, updated live from Rust.
- Manual verification on Windows before phase close.

## Phase 5: Persistent Data Management ✓
**Goal**: Import flashcard stacks from markdown files and export all stacks back, using native file dialogs on desktop.

- Define the markdown file format: `## Stack Name` headings + GFM pipe table per stack; document in `docs/markdown-format.md`.
- Create `lib/persistent_data` (libB): `markdown_io.rs` (parse + serialize via `pulldown-cmark`), `file_io.rs` (open/save dialogs via `rfd`, WASM no-op stubs).
- Wire Import and Export buttons in `StudyPage`; call `persistent_data::init(&ui)` from `src/main.rs`.
- Verify round-trip on Windows: import → edit → export → re-import without data loss.

## Phase 6: Vocabulary Study Mode and Exercise Generation ✓
**Goal**: Users can author vocabulary lessons (words with kanji, spelling, meaning, type, tense, and example sentences), then generate decoupled flashcard stacks from those lessons on demand. The Review Page gains a read-only matching exercise. Editing flashcard stacks does not affect the vocabulary lesson database, and vice versa.

**New libraries**:
- `lib/exercise_generator` (libD) — pure Rust transformer service; no Slint, no `build.rs`, no `init()`. Converts `VocabularyLesson` structs into `FlashcardStackData` structs. See `.claude/rules/libD-code-style.md` for the full pattern.
- `lib/vocabulary` (libA) — vocabulary lesson CRUD UI + Rust backend; persists to `vocabulary.json`; handles vocabulary markdown import/export (separate format from the basic flashcard markdown in `lib/persistent_data`).

**Data flow** (decoupled databases):
```
[vocabulary.json]  ──on demand──>  [FlashcardExerciseTransformer (libD)]  ──>  [stacks.json]
    (vocabulary DB)                    pure Rust, stateless                       (exercise DB)
```

**Key rules**:
- libD owns the plain Rust domain model structs (`VocabularyLesson`, `VocabularyWord`, `FlashcardStackData`). Word bank and sentence bank are internal aggregates of these structs — no separate UI or persistence.
- All type conversions (Slint ↔ plain Rust) happen in `lib/vocabulary/src/lib.rs` — never inside libD.
- "Generate Exercises" / "Re-create Exercises": clears `stacks.json` and regenerates from vocabulary data; vocabulary database is unchanged.
- Kanji duplication rule: if a word has a kanji field, two flashcard cards are created — one with kanji front, one with spelling front; both share the same explanation back.
- Vocabulary markdown format — lessons delimited by `## Lesson Name` headings; each word entry opened by a `### <spelling>` subheading; `kanji:`, `meaning:`, `type:`, `tense:`, `example:` as key-value lines below (tense and example may repeat) — is owned by `lib/vocabulary`. It is separate from and must not be mixed into the basic flashcard markdown format in `lib/persistent_data`.

**Modules**:
- `lib/exercise_generator/src/models.rs` — `VocabularyLesson`, `VocabularyWord`, `TenseEntry`, `FlashcardStackData`, `FlashcardCardData`.
- `lib/exercise_generator/src/transformer.rs` — `Transformer<S,T>` trait, `ExerciseRequest` enum, `ExerciseOutput` enum.
- `lib/exercise_generator/src/service.rs` — `ExerciseGeneratorService` dispatcher.
- `lib/exercise_generator/src/flashcard_transformer.rs` — `FlashcardExerciseTransformer` impl + unit tests.
- `lib/vocabulary/ui/` — `VocabularyLessonModel`, `VocabularyWordModel` Slint structs, `VocabularyAppLogic` global, lesson list, word form (spelling, kanji, meaning, type, tense list, example list).
- `lib/vocabulary/src/lib.rs` — vocabulary persistence, CRUD handlers, vocabulary markdown import/export, `on_generate_exercises_clicked` (calls libD, updates flashcard lib).
- `lib/vocabulary` depends on `lib/flashcard` as a workspace Rust dep: the `on_generate_exercises_clicked` handler reads from `VocabularyAppLogic`, calls `ExerciseGeneratorService`, then writes to `FlashcardAppLogic` and calls `flashcard::save_stacks()`. All Slint ↔ Rust type conversions stay in `lib/vocabulary/src/lib.rs`.

**Study Page navigation**:
- `StudyPage` has three topic tabs: **Vocabulary** (index 0), **Grammar** (index 1), **Reading** (index 2). The former Flashcard tab is removed; flashcard management is now accessible from inside `VocabularyPage`.
- `VocabularyPage` hosts an action bar with three tab views (**Lesson** / **Exercise** / **Flashcard**) and one direct action (**Import Lesson**). Full navigation design is in `speckit.specify.prompt.md` § *Urgent requirements change*.
- Grammar and Reading topics remain placeholders at this phase.

**Vocabulary Review Mode** (Review Page):
- The Review Page shows the flashcard stack list in read-only mode — no add/delete/edit controls.
- Selecting a stack launches a matching exercise: all flashcard fronts and backs for that stack are displayed as separate tiles in randomised order; the user clicks a front tile then clicks its matching back tile; matched pairs lock and are visually distinguished; the exercise passes when all pairs are matched.
- Interaction model: click-to-select (works on desktop, WASM, and mobile without drag-and-drop).
- The matching exercise UI is implemented in `lib/vocabulary/ui/` as a `MatchingExerciseView` component.

**Milestone completion check**: create a vocabulary lesson → add words with and without kanji → click Generate Exercises → confirm flashcard stacks appear with kanji duplication → manually edit a flashcard stack → click Re-create → confirm stacks reset to vocabulary-derived data with vocabulary lesson unchanged → open Review Page → select a stack → complete the matching exercise → confirm all pairs resolve correctly.

## Phase 6.D: Default Vocabulary Data ✓
**Goal**: Bundle N5 vocabulary datasets with the app so users can study immediately without manual data entry. Three datasets ship as embedded source files; they are never overwritten at runtime.

- **Data authoring**: Create and version-control six files in `lib/vocabulary/ui/data/` — `n5_verbs.md`, `n5_adjectives.md`, `n5_vocabulary.md`, and one JSON equivalent for each. Each file is a complete, valid vocabulary lesson with full word entries (spelling, kanji where applicable, meaning, type, at least one tense, at least one example sentence where grammatically relevant).

- **Auto-load on first launch**: In `lib/vocabulary/src/lib.rs`, at `init()` startup detect whether `vocabulary.json` is absent. If absent, parse the three embedded datasets using `include_str!()` and save them to `vocabulary.json`. If `vocabulary.json` already exists, skip auto-load (user may have previously removed defaults intentionally).

- **Restore Defaults UI + handler**: Add `callback restore-defaults-clicked()` to `VocabularyAppLogic`. Add a `CommonBtn "Restore Defaults"` above `LessonStackList` in the Lesson view of `VocabularyPage`. The Rust handler clears all current lessons from memory and disk, then reloads and saves the three embedded datasets.

- No new libraries or crates. Uses `include_str!()` (Rust built-in) for embedding and existing `serde_json` for parsing. Data folder convention (`lib/[library]/ui/data/`) is documented in `architecture.md`.

---
## ✅ Milestone 2 Complete — Phases 3 · 4 · 5 · 6 · 6.D · 6.R · 6.S · 6.B · 6.T · R1
*Tagged: v2.0.0 / Milestone2*

---

## Phase 7: Mobile Support (Android + Swipe Gestures)
**Goal**: Deploy on Android; add swipe navigation. Placed here so all subsequent feature phases are built mobile-ready from the start.

- Configure Android build (`cargo apk`); verify blank window on Android emulator.
- Implement `SwipeArea` reusable component in `lib/styles/ui/` using `TouchArea` delta events; expose swipe-left, swipe-right, swipe-down callbacks.
- Wire `SwipeArea` into the study session view for card navigation and session close.
- Add Android file-picker no-op stub in `lib/persistent_data`, gated `#[cfg(target_os = "android")]`.
- Audit all interactive elements for minimum 44 dp touch targets.
- Note: Slint's Android backend is included via Slint's feature flags — no additional crate dependency.

## Phase 8: Optimization & Testing
**Goal**: Harden the application after Phases 1–7 — performance, coverage, and cross-platform compliance across Windows, WebAssembly, and Android.

- Optimize rendering performance across all three targets.
- Add Rust unit tests for core logic (flashcard CRUD, study mode state).
- Add Rust integration tests for data persistence (markdown round-trip).
- Ensure compliance with constitution best practices (UI separation, modularity, `@styles` usage).
- Document testing results and performance benchmarks.

## Phase 9: Analytics
**Goal**: Track study sessions and display per-stack progress using pure Slint chart components — no charting library.

- **Library type**: libA — `lib/analytics/`.
- Session log stored as JSON (`sessions.json`); each entry: date, stack name, total cards, known count.
- Slint components: `ProgressBarChart` (horizontal bar, known% vs unknown%) and `SessionHistoryChart` (vertical bars over last N sessions), implemented via proportional `Rectangle` widths/heights.
- Rust modules: `session_log.rs` (append + read), `stats.rs` (aggregate per stack).
- Add `AnalyticsPage` with navigation in `MainWindow`; record a session entry on study session close.
- No new third-party dependencies beyond `serde`/`serde_json`.

## Phase 10: Grammar Study Mode
**Goal**: Grammar lessons with three exercise types and pass/fail tracking per exercise.

- **Library type**: libA — `lib/grammar/`.
- Data models: `GrammarLessonModel { title, structure, sentences, known, exercises-passed }` and `SentenceModel { japanese, meaning, tokens }` — `tokens` is optional; if absent, the reconstruction exercise is skipped for that sentence.
- Persistence: `grammar.json` via `serde`/`serde_json`, same pattern as `stacks.json`.
- Slint components: `GrammarPage`, `LessonList`, `MatchingExercise` (drag-to-match), `ReconstructExercise` (drag word-chip blocks), `FillBlankExercise` (inline `TextInput`).
- Rust modules: `exercise_engine.rs` (generate shuffled exercises from lesson data), `scoring.rs` (pass/fail), `persistence.rs`.
- Japanese tokenization uses `SentenceModel.tokens` supplied by the teacher — no NLP library.
- When all exercises for a lesson pass, mark the lesson `known`.

## Phase 11: Audio (TTS + Playback)
**Goal**: Play Japanese TTS for flashcard words and grammar sentences. Depends on Phase 10 (`GrammarPage` must exist before wiring audio to it).

- **Library type**: libB — `lib/audio/`, no Slint UI.
- Desktop: `tts = "0.26"` (Windows SAPI / macOS AVSpeechSynthesizer), gated `#[cfg(not(target_arch = "wasm32"))]`.
- WASM: `web_sys::SpeechSynthesis`, gated `#[cfg(target_arch = "wasm32")]`.
- Public API: `pub fn speak(text: &str)` — fire-and-forget; errors logged, never panic.
- Add "Listen" button to `Flashcard` and to `GrammarPage` sentence rows; wire `callback speak-requested(text: string)` through each library's `AppLogic` global to `audio::init()`.
- Users must install a Japanese TTS voice at OS level (Windows: Settings → Language → Japanese → Voice).

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tts = "0.26"

[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys = { version = "0.3", features = ["SpeechSynthesis", "SpeechSynthesisUtterance", "Window"] }
```

## Phase 12: Listening Study Mode & Review Mode
**Goal**: Listening exercises that test comprehension using TTS playback. Depends on Phase 11.

- **Study Listening Mode**: a toggle in the study session view switches to audio-only (hide Japanese text, auto-play TTS on card advance); user can still tap to reveal.
- **Review — Multiple Choice**: pick a random card or sentence, speak it via `audio::speak`, display 3–4 text choices (correct + distractors from the same stack), user selects.
- Distractor selection logic in `lib/flashcard/src/study_mode.rs` (flashcard) and `lib/grammar/src/` (sentences).

# Deliverables
- Each phase delivers a working, tested increment of the application.
- Final deliverable includes:
  - A working Windows + WebAssembly + Android application with flashcard management, grammar study, analytics, and audio.
  - `lib/styles` design token library consumed by all components.
  - Documentation of architecture, modules, and usage.
  - Rust test suite covering core features (CRUD, exercise engine, persistence).
  - Cloud sync deferred to a future iteration.
