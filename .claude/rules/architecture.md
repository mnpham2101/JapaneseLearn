# General Architecture
- Follow MVC architecture, but keep the common folder structure as other Slint-Rust applications: 
  - Slint components must be in together, but keep the folder name as 'ui/components/'. The entry Slint component must be in folder 'ui'
  - All public model data struct should be defined in 'ui/model/'
  - All backend business logic should be in 'src/'
- If the Slint components and its Rust backend logic are built as library, keep them in '/lib'. 
- Folder structure: 
```
src/
  main.rs                       # entry point only — no business logic here
ui/
  main_window.slint             # root window, page routing
  components/                   # main application components - one component per file
  ui/styles                     # reusable style definitions
  model/                        # main application data models
  pages/                        # page-level components (study_page, review_page)
lib/
  libA/                           # Slint component and backend logic built as library
    src/lib.rs                    # entry point to init function
    src/*.rs                      # Rust backend logic supporting UI manipulation
    ui/components                 # reusable widgets — one component per file 
    ui/model                      # reusable data models written in Slint
    ui/main_lib.slint             # entry slint library file - export the library components and models                
    build.rs                      # entry point to build library
    Cargo.toml                    # define building codes as package
  libB/                           # Rust logic built as service library
    src/lib.rs                    # Rust backend logic
    src/*.rs                      # Rust backend logic, could include helper functions, classes, etc 
    main.rs                       # entry point to build library
    Cargo.toml                    # define building codes as package
  libD/                           # Pure Rust data transformation service — no Slint, no build.rs, no ui/
    src/lib.rs                    # Public API: re-exports transformer trait, service, and all output types
    src/models.rs                 # Domain model structs (Slint-free, serde if needed)
    src/transformer.rs            # Transformer<S,T> trait, ExerciseRequest enum, ExerciseOutput enum
    src/service.rs                # ExerciseGeneratorService dispatcher
    src/<name>_transformer.rs     # Concrete transformer implementation(s) — one per (S→T) pair
    Cargo.toml                    # Pure Rust deps only (serde if models need it) — NO slint, NO slint-build
  libC/                           # Pure Slint design library — no Rust backend
    ui/tokens.slint               # color palette, typography, spacing constants
    ui/animations.slint           # easing curves, duration constants
    ui/main_lib.slint             # re-exports all design tokens for @libC imports
    build.rs                      # compiles ui/main_lib.slint as a library
    Cargo.toml                    # name = "libC", links key, slint-build only (no slint dep)
test/
  [library_name]/                 # one workspace-member crate per library under test
    tests/
      [page_name].rs              # integration tests for a specific page (e.g. study_page.rs)
    Cargo.toml                    # depends on the library crate; slint features = ["testing"]
build.rs
Cargo.toml
```

> For a visual overview of all layers and dependencies, see `.claude/rules/architecture_diagram.puml`.

## Library Types

| Type | Has Rust backend | Has Slint UI | Use when |
|---|---|---|---|
| **libA** | Yes (`src/lib.rs` + `init()`) | Yes (`ui/components/`, `ui/model/`) | UI and backend are tightly coupled (CRUD, exercise engines) |
| **libB** | Yes (service, no `init()` required) | No | Platform service with no UI: file I/O, audio TTS |
| **libC** | No | Yes (tokens/animations only) | Design system: tokens, color palette, typography, spacing, animation curves |
| **libD** | Yes (pure Rust, generic `Transformer<S,T>` interface + service dispatcher) | No | Data transformation service: a generic `Transformer<S, T>` trait, concrete implementations (one per source→target pair), and a `ExerciseGeneratorService` dispatcher that selects the right transformer at runtime from an `ExerciseRequest`; no Slint types, no platform deps, no `init()` |

> A libC library has no `src/` directory and no Rust `init()` function. Its `Cargo.toml` lists only `slint-build` as a build-dependency.  
> A libD library has no `build.rs`, no `ui/` directory, and no `init()` function. It is pure Rust computation — a generic interface (`Transformer<S, T>` trait), concrete transformer structs, and a stateless service dispatcher. Adding a new exercise type never modifies existing transformer code (SOLID Open/Closed). It is called from a libA `init()` handler, never from `src/main.rs`. See `.claude/rules/libD-code-style.md` for the full pattern with code examples.  
> All other libraries import design tokens from `@styles` (the libC instance) — never hardcode colors, sizes, or durations.

## Planned Library Catalogue

| Crate | Type | Responsibility |
|---|---|---|
| `lib/flashcard` | libA | Flashcard CRUD UI, stack management, session persistence |
| `lib/persistent_data` | libB | Local file import/export (markdown parsing, file dialogs) |
| `lib/styles` | libC | Design tokens, color palette, typography, spacing, animation curves |
| `lib/analytics` | libA | Study-session logging, statistics computation, chart Slint components |
| `lib/grammar` | libA | Grammar lesson models, exercise UI (matching, reconstruct, fill-blank), exercise engine |
| `lib/audio` | libB | TTS and audio playback — OS voice engine on desktop, Web Speech API on WASM |
| `lib/vocabulary` | libA | Vocabulary lesson CRUD UI, lesson persistence to `vocabulary.json`; word bank and sentence bank are internal data structures (not visible UI) used by exercise generation |
| `lib/exercise_generator` | libD | Transformer service: converts `VocabularyLesson` data into exercise datasets (`FlashcardStackData`, future output types); databases are decoupled — conversion is on-demand only |

## Platform-Specific Notes

- `rfd` (file dialogs) and `std::fs` calls **must** be gated with `#[cfg(not(target_arch = "wasm32"))]`.  
- Audio on desktop uses the `tts` crate (wraps Windows SAPI / macOS AVSpeechSynthesizer). Users must have a Japanese TTS voice installed at the OS level.  
- Audio on WebAssembly uses the browser's Web Speech API via `web_sys` — no `tts` crate needed.  
- Android builds use Slint's built-in Android backend; `rfd` is not supported on Android — a separate file-picker integration is required.  
- Swipe gestures (mobile/touch) are implemented with Slint's `TouchArea` pointer delta events — no external gesture library needed.