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