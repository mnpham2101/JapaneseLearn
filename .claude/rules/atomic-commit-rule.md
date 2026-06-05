# Atomic Commit Rule

## Core Principle
Each commit represents exactly **one logical change** that independently:
- Compiles with zero errors (`cargo build` green)
- Passes all existing tests with no regressions (`cargo test`)
- Can be reviewed in isolation

## Scope — One Commit, One Thing

| Change type | Scope of one commit |
|---|---|
| Slint callback declaration | The `.slint` declaration only — Rust handler is a separate commit |
| New Slint component or property | That component or property only |
| New Rust function | That function body only — caller registration is a separate commit |
| `init()` callback registration | The `on_xyz(...)` call only — after the handler is already committed |
| Build config (build.rs) | One crate's build.rs change at a time |
| File migration (import paths) | One library's files per commit |

## Chain Call Rule

When feature `xyz` requires `funcA` (existing) to call `funcB`, which calls `funcC`:

```
1. Implement funcC  →  build → test → commit "feat: implement funcC"
2. Implement funcB (calls funcC)  →  build → test → commit "feat: implement funcB"
3. Wire funcA → funcB  →  build → test → clippy → commit "feat: complete xyz, wire funcA to funcB"
```

The final commit adds only the **call site** — no new logic. New logic always lives in an earlier commit.

## Warning Policy — Errors vs. Warnings

| Check | Intermediate chain commit | Chain-completing commit |
|---|---|---|
| `cargo build` (zero errors) | **Required** | **Required** |
| `cargo test` (no regressions) | **Required** | **Required** |
| `cargo clippy` (zero warnings) | Not required | **Required** |

**Intermediate commits in a chain may have dead-code warnings** when a new helper function has not yet been called by any handler. That is acceptable. The commit that wires the final call site naturally eliminates the warning and must be zero-warning.

Never suppress warnings with `#[allow(dead_code)]` — that defers a real problem. Do not add artificial calls to `init()` startup just to silence a warning; let the chain-completing commit clear it naturally.

## Slint-Developer Specifics
- A callback **declaration** in `.slint` builds without its Rust handler.
  The handler is registered in `init()` only after being committed by rust-developer.
- Each new component, property group, or callback declaration group = one commit.

## Rust-Developer Specifics
- Implement helper functions before registering them as callback handlers in `init()`.
- One `on_xyz(...)` registration per commit after the handler body is committed.
- Do not add artificial calls to `init()` startup to avoid dead-code warnings — let the chain-completing commit resolve them.

## Build Verification

Run before every commit:
```powershell
cargo build --bin japanese_learn                              # required always
cargo fmt                                                     # required always
cargo test -p flashcard-tests -- --test-threads=1            # required always
cargo clippy --bin japanese_learn                             # required only on chain-completing commits
```
