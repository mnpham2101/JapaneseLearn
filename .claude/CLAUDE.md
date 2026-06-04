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

### Build troubleshooting

If `cargo build` fails with `LNK1201` (PDB locked by a running process):
```powershell
taskkill /F /IM japanese_learn.exe 2>$null
Remove-Item "target\debug\deps\japanese_learn.pdb" -ErrorAction SilentlyContinue
Remove-Item "target\debug\japanese_learn.pdb" -ErrorAction SilentlyContinue
cargo build
```

## Project Structure

```
src/
  main.rs                       # entry point only — no business logic here
ui/
  main_window.slint             # root window, page routing
  components/                   # application UI components — one component per file
  model/                        # Slint data model structs (FlashcardModel, etc.)
  pages/                        # page-level components (study_page, review_page)
lib/                            # optional: Slint+Rust components built as libraries
  <libname>/
    src/lib.rs                  # init function, Rust backend logic
    ui/                         # library's own Slint components and models
    build.rs
    Cargo.toml
.claude/
  agents/slint-developer.md     # Slint developer agent (use for all .slint work)
  agents/task-manager.md        # task-manager agent (use for multi-task planning)
  rules/slint-code-style.md     # Slint coding rules, naming conventions, patterns
  rules/rust-code-style.md      # Rust coding rules
  skill/implement-tasks/SKILL.md  # task follow-up and commit workflow
.github/prompts/
  speckit.tasks.prompt.md       # active task list
  speckit.constitution.prompt.md  # general programming practices + architecture
```

## Coding Standards

Full rules live in the dedicated files — do not duplicate here:
- **Slint/UI**: `.claude/rules/slint-code-style.md`
- **Rust**: `.claude/rules/rust-code-style.md`

Key invariants (always apply):
- One `.slint` component per file; components in `ui/components/`, data models in `ui/model/`, pages in `ui/pages/`.
- Slint component naming — PascalCase with functional suffix: `*Btn`, `*TxtBox`, `*Container`, `*Page`.
- Property bindings over imperative callbacks; `<=>` for two-way state sync.
- Hardcoded models first; integrate dynamic data only after a successful build.
- No Rust logic inside `.slint` files.

## Git & Commit Discipline

- Commits **must** be atomic — one objective per commit.
- Run `cargo fmt`, `cargo clippy`, and `cargo build` before every commit.
- Commit message format:
  ```
  type: description
  TaskId: Task X.Y
  ```
- Never include unrelated changes in the same commit.

## Implementation Workflow

1. Pick the next open task from `.github/prompts/speckit.tasks.prompt.md`.
2. Use the **slint-developer** agent (`.claude/agents/slint-developer.md`) for all `.slint` work.
3. Follow task execution rules in `.claude/skill/implement-tasks/SKILL.md`.
4. Validate: `cargo build` passes, UI tested manually.
5. Suggest a commit message referencing the task ID; await review before the next task.

## Reuse & Simplicity

- Prefer reusing existing components and APIs over adding new ones.
- Do not add properties, functions, or components unless strictly required by the task.
- Three similar lines is better than a premature abstraction.

## Constitution Reference

Full project constitution: `.specify/memory/constitution.md` (v1.0.0, ratified 2026-06-01).
