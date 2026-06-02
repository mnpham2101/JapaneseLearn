# Japanese Learn — CLAUDE.md

Japanese language flashcard application built with **Rust + Slint**. Targets desktop (Windows) and WebAssembly.

## Build & Run

```powershell
cargo run              # desktop
cargo fmt              # format (run before every commit)
cargo clippy           # lint (run before every commit)
cargo test             # unit + integration tests

# WebAssembly
wasm-pack build --release --target web
python3 -m http.server  # serves at http://localhost:8000
```

Prerequisites: Rust 1.96+, `cargo install wasm-opt` for web builds.

## Project Structure

```
src/
  main.rs              # entry point only — no business logic here
ui/
  main_window.slint    # root window, page routing
  components/          # reusable widgets (common_button, nav_button, common_list)
  pages/               # page-level components (study_page, review_page)
```

## Slint Rules (MUST follow)

- UI definitions **must** live in `.slint` files — never mix Rust logic into markup.
- Each component **must** be in its own `.slint` file; import where needed.
- Use **property bindings** instead of imperative updates; react to changes with the `changed` keyword.
- Keep component hierarchies **shallow**; prefer reusable components over duplication.
- Common components (buttons, lists) **must** define base properties/behaviors and be extended for specific cases.
- Build with **hardcoded default models first**; only integrate dynamic data after a successful build.
- No unnecessary animations or large assets — optimize for lightweight rendering.

## Rust Rules (MUST follow)

- Favor `let` over `let mut`; mutate only when essential.
- Error handling via `Result`/`Option` — no panics in normal control flow.
- Separate concerns into distinct modules:
  - **UI logic** — Slint event handling and property updates
  - **Business logic** — core flashcard/study functionality
  - **Data access** — JSON/file persistence
- Document public APIs with `///` doc comments and examples.
- Use only well-maintained crates; keep dependencies minimal.

## Git & Commit Discipline

- Commits **must** be atomic (one objective per commit) and pass `cargo build` + `cargo test`.
- Commit message format: `type: description [Task X.Y]` where X = phase, Y = task number.
- Run `cargo fmt` and `cargo clippy` before every commit.
- Never include unrelated changes in the same commit.

## Implementation Workflow

1. Select a task from the current `specs/.../tasks.md`.
2. Implement in Rust + Slint following all constitution rules.
3. Validate: build for Windows **and** WebAssembly, run tests, manually test UI.
4. Commit with descriptive message referencing the task ID.
5. Review before proceeding to the next task.

## Reuse & Simplicity

- Prefer reusing existing components and APIs over adding new ones.
- Do not add properties, functions, or components unless strictly required by the task.
- Three similar lines is better than a premature abstraction.

## Constitution Reference

Full project constitution: [`.specify/memory/constitution.md`](.specify/memory/constitution.md) (v1.0.0, ratified 2026-06-01).
Speckit workflow prompts: [`.github/prompts/`](.github/prompts/).

