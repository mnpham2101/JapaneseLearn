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
  libC/                           # Pure Slint design library — no Rust backend
    ui/tokens.slint               # color palette, typography, spacing constants
    ui/animations.slint           # easing curves, duration constants
    ui/main_lib.slint             # re-exports all design tokens for @libC imports
    build.rs                      # compiles ui/main_lib.slint as a library
    Cargo.toml                    # name = "libC", links key, slint-build only (no slint dep)
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

> A libC library has no `src/` directory and no Rust `init()` function. Its `Cargo.toml` lists only `slint-build` as a build-dependency.  
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

## Platform-Specific Notes

- `rfd` (file dialogs) and `std::fs` calls **must** be gated with `#[cfg(not(target_arch = "wasm32"))]`.  
- Audio on desktop uses the `tts` crate (wraps Windows SAPI / macOS AVSpeechSynthesizer). Users must have a Japanese TTS voice installed at the OS level.  
- Audio on WebAssembly uses the browser's Web Speech API via `web_sys` — no `tts` crate needed.  
- Android builds use Slint's built-in Android backend; `rfd` is not supported on Android — a separate file-picker integration is required.  
- Swipe gestures (mobile/touch) are implemented with Slint's `TouchArea` pointer delta events — no external gesture library needed.