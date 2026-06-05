---
paths: 
  - src/**/*.rs
  - lib/**/*.rs
---

# Rust Code Style & Patterns

This file is the **primary Rust reference** for the rust-developer. Read it at the start of any Rust task. When a section says "see X", open that file for the full pattern — do not guess.

## Step 0 — Identify the Library Type Before Writing Code

**Always check the Planned Library Catalogue in `.claude/rules/architecture.md` first.** Every feature in this project is pre-assigned to a crate and a lib type. Find the crate, find the lib type, then apply the matching pattern below.

Do not infer the lib type from the code you are about to write — the catalogue is the source of truth.

## Library Types and When to Apply Each

| Lib type | Service goal | Rust pattern | Full guide |
|---|---|---|---|
| **libA** | Feature with Slint UI + Rust callbacks: CRUD, session state, exercise UI | Library Callback Pattern (with `init()`) | this file |
| **libB** | Platform service with no Slint UI: file I/O, audio TTS, OS dialogs | Service module (no `init()` required; `#[cfg]` gates for WASM) | this file + `architecture.md` |
| **libD** | Data transformation pipeline: source database → target database, pure computation | `Transformer<S,T>` trait + service dispatcher | **`.claude/rules/libD-code-style.md`** |

> **libC** (design tokens, typography, spacing) has no Rust code — it is owned entirely by the slint-developer.

If a future service goal does not match any row in this table, **do not create a new crate without consulting the project owner.** The catalogue in `architecture.md` must be updated first.

---

# Rust Best Practices
- Immutability **must** be favored; `let` **must** be used over `let mut` unless mutation is essential.
- Error handling **must** use `Result` and `Option`; panics **must not** be used for normal control flow.
- Code **must** be organized with modules and crates; functions **should** remain small and focused.
- `cargo fmt` and `clippy` **must** be run regularly to enforce style and catch common mistakes.
- Public APIs **must** be documented with `///` doc comments and examples.
- Third‑party libraries **must** be integrated via `Cargo.toml` carefully; only well‑maintained crates **should** be used.
- UI logic, business logic, and data access **must** be separated into distinct modules: UI logic handles Slint event handling and property updates; business logic encapsulates core application functionality; data access manages databases or external APIs. Mixing concerns across these modules **must not** occur.

## Library Callback Pattern (Slint library init)

All callback wiring logic **must** be implemented inside `lib/<name>/src/lib.rs` in the `init()` function — **never** in `src/main.rs` of the main application. `src/main.rs` **must** remain entry-point only: create the window, call each library's `init()`, and call `window.run()`.

Slint library callback logic is accessed via a **global singleton** declared in the library's `.slint` files. The `init()` function is generic and requires the root component to expose that global:

```rust
// lib/<name>/src/lib.rs
slint::include_modules!();  // generates MyLibGlobal and other types from the library's .slint

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> MyLibGlobal<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<MyLibGlobal>();
    let ui_weak = ui.as_weak();

    logic.on_some_callback(move || {
        let ui = ui_weak.unwrap();
        let logic = ui.global::<MyLibGlobal>();
        // read/write properties on the global to update app state
        logic.set_some_property(new_value);
    });
}
```

The matching `src/main.rs` entry point:
```rust
// src/main.rs — entry point only
slint::include_modules!();

fn main() {
    let window = MainWindow::new().expect("Failed to create MainWindow");
    my_library::init(&window);  // all callback wiring happens inside the library
    window.run().expect("Failed to run");
}
```

Key rules:
- The global **must** be declared in a `.slint` file inside the library (e.g., `lib/<name>/ui/app_logic.slint`) and re-exported from the library's entry slint file.
- Slint components fire callbacks and bind properties to this global; Rust registers handlers on it via `init()`.
- `MainWindow` does **not** forward callbacks to `src/main.rs`; those forwarding lines are removed.
- Persistence, CRUD logic, and all side effects belong in `lib/<name>/src/lib.rs` (or sub-modules), never in `src/main.rs`.

## libD — Data Transformation Service

The catalogue assigns `lib/exercise_generator` to libD. Its job is the **lesson-database → exercise-database pipeline**: convert raw lesson data into structured exercise datasets on demand — no UI, no file I/O, no platform calls.

If the catalogue assigns a different crate to libD (e.g. a future grammar-to-quiz transformer), the same pattern applies to that crate. Each libD crate is independent; they do not share code.

**Confirm a crate is libD before applying this pattern** — check all four:
1. Input and output are plain Rust structs (no `SharedString`, no `ModelRc`).
2. No OS calls — no `std::fs`, no `rfd`, no `tts`, no `#[cfg(target_arch)]` guards.
3. Source database and target database must stay decoupled — conversion is user-triggered, not automatic.
4. Multiple output formats from the same source are likely — adding a new format must not modify existing code.

**Read `.claude/rules/libD-code-style.md` before writing any libD code.** It contains the complete four-component design (`models.rs`, `transformer.rs`, `service.rs`, `<name>_transformer.rs`), all code templates, the SOLID rationale, the extension recipe, and the integration pattern showing how a calling libA `init()` handler invokes the service.