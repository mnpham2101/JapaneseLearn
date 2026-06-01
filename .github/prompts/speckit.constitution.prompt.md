---
agent: speckit.constitution
---

/speckit.constitution

# Slint Best Practices
- UI definitions **must** be separated into `.slint` files; Rust logic **must not** be mixed with UI markup.
- Property bindings **must** be used instead of imperative updates wherever possible.
- The `changed` keyword **must** be used to react to property changes.
- Component hierarchies **should** remain shallow; reusable custom components **must** be preferred over duplication.
- Common UI components (buttons, lists, etc.) **must** be defined with base properties and behaviors, and **should** be extended for specific use cases.
- Specific UI components (e.g., custom buttons, list items) **must** be defined in separate `.slint` files and imported where needed.
- Components requiring data models (e.g., list views, `HorizontalView`) **must** have default hardcoded models first; the program **must** build successfully before integrating dynamic data sources. Code **must** be committed after each successful UI component implementation.
- Rendering **must** be optimized for lightweight performance; unnecessary animations or large assets **must not** be used.
- UI responsiveness **must** be tested across desktop, mobile, and WebAssembly targets.

# Rust Best Practices
- Immutability **must** be favored; `let` **must** be used over `let mut` unless mutation is essential.
- Error handling **must** use `Result` and `Option`; panics **must not** be used for normal control flow.
- Code **must** be organized with modules and crates; functions **should** remain small and focused.
- `cargo fmt` and `clippy` **must** be run regularly to enforce style and catch common mistakes.
- Public APIs **must** be documented with `///` doc comments and examples.
- Third‑party libraries **must** be integrated via `Cargo.toml` carefully; only well‑maintained crates **should** be used.
- UI logic, business logic, and data access **must** be separated into distinct modules. UI logic **must** handle Slint interactions; business logic **must** encapsulate core functionality; data access **must** manage databases or APIs. Mixing concerns **must not** occur. In particular, **must** create a separate module for UI logic that handles all interactions with the Slint UI components such as event handling and property updates. **must** create separate modules for business logic that encapsulate the core functionality of the application, and data access that manages interactions with databases or external APIs. This separation of concerns will lead to a more modular and maintainable codebase, allowing for easier testing and future enhancements.

# General Programming Practices
- Code **must** be modular and reusable, with clear separation of concerns.
- Each task **must** have a defined scope and objectives before implementation.
- Commits **must** be atomic and descriptive; unrelated changes **must not** be included in the same commit.
- Each commit **must** pass builds and tests before being pushed.
- Variable and function names **must** be descriptive; consistent naming conventions **must** be followed across files.
- Readability **must** be prioritized over cleverness; maintainability **must** be ensured.
- Automated tests **must** be implemented for core logic and UI interactions.
- Cross‑platform compatibility **must** be ensured; OS‑specific assumptions **must not** be made.
- Dependencies **must** be kept minimal to reduce binary size and deployment complexity.
- Version control (e.g., Git) **must** be used with meaningful commit messages.
