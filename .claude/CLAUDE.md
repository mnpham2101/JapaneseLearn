# Japanese Learn — CLAUDE.md

Rust + Slint Japanese flashcard app targeting desktop (Windows) and WebAssembly.

## Build & Run

```powershell
cargo run              # desktop
cargo fmt              # format — run before every commit
cargo clippy           # lint — run before every commit
cargo test             # unit + integration tests

# WebAssembly
wasm-pack build --release --target web
python3 -m http.server
```

Always use `--bin japanese_learn` — parallel bin+cdylib builds share a PDB name.  
For LNK1201 (PDB locked): see `.claude/skill/implement-tasks/SKILL.md` Step 3.

## Project Structure

```
src/main.rs            # entry point only
ui/                    # Slint components (components/, model/, pages/, styles/)
lib/                   # Slint+Rust libraries (libA) and Rust-only services (libB)
test/                  # automated tests
.claude/               # agents, rules, skills
.github/prompts/       # task list and constitution
```

## Implementation

Pick the next open task from `.github/prompts/speckit.tasks.prompt.md`. For multi-task work, use the **task-manager** agent; follow the [implement-tasks skill](.claude/skill/implement-tasks/SKILL.md).

## Rules & Standards

| Concern | File |
|---|---|
| Architecture & folder layout | `.claude/rules/architecture.md` |
| Slint/UI patterns and library setup | `.claude/rules/slint-code-style.md` |
| Rust patterns and callback wiring | `.claude/rules/rust-code-style.md` |
| General coding practices (incl. minimal dependencies) | `.claude/rules/general-programming-practice.md` |
| Commit message format | `.claude/rules/commit-msg-format.md` |
| Atomic commit rules | `.claude/rules/atomic-commit-rule.md` |
| Test format and templates | `.claude/rules/slint-test-format.md` |
