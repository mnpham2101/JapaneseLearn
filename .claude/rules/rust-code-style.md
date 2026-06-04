---
paths: 
  - src/**/*.rs
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