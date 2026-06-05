# General Programming Practices
- Code **must** be modular with clear separation of concerns; reuse existing functions and components before adding new ones.
- Commits **must** be atomic: one task or fix per commit; unrelated changes **must not** be included in the same commit.
- Every commit **must** pass `cargo fmt`, `cargo clippy` (zero warnings), and `cargo build` before being pushed.
- Variable and function names **must** be descriptive; consistent naming conventions **must** be followed across files.
- Readability **must** be prioritized over cleverness.
- Cross‑platform compatibility **must** be ensured; OS‑specific code **must** be gated with `#[cfg(target_os = "...")]` or `#[cfg(not(target_arch = "wasm32"))]` — never assume the host platform.
- Dependencies **must** be kept minimal; a new crate requires justification — prefer `std`, Slint built-ins, or existing workspace deps first.
