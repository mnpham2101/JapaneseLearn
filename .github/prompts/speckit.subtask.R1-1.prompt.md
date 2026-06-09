# Subtask R1.1 — Create lib/common_component libC scaffold

**Agent**: slint-developer  
**Parent task**: R1.1  
**Depends on**: none  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `lib/common_component/` exists as a pure Slint libC folder (no Cargo.toml, no src/, no build.rs) with an empty entry file `common_component_lib.slint`. The `@common_component` path is registered in the three consumer `build.rs` files (root, flashcard, vocabulary). The build passes with zero errors — no `.slint` file imports `@common_component` yet, so nothing breaks.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/common_component/common_component_lib.slint` | create | empty entry file (no exports yet) |
| `lib/common_component/components/.gitkeep` | create | placeholder so folder exists in git |
| `build.rs` (root) | modify | add `@common_component` path to `library_paths` |
| `lib/flashcard/build.rs` | modify | add `@common_component` path to `library_paths` |
| `lib/vocabulary/build.rs` | modify | add `@common_component` path to `library_paths` |

## Components / Modules

**`lib/common_component/common_component_lib.slint`** — entry file for the library. Empty for now; will export `CommonBtn`, `CommonList`, `CommonGrid` in subsequent subtasks.

```slint
// Entry file — exports added by R1.2, R1.3, R1.4
```

## build.rs Changes

### root `build.rs`

Add `common_component` to `library_paths`:

```rust
let common_component_path = manifest_dir.join("lib/common_component/common_component_lib.slint");
let library_paths = std::collections::HashMap::from([
    ("styles".to_string(), styles_path),
    ("common_component".to_string(), common_component_path),
]);
```

### `lib/flashcard/build.rs`

Add `common_component` to `library_paths`:

```rust
let common_component_path = manifest_dir.join("../../lib/common_component/common_component_lib.slint");
let library_paths = std::collections::HashMap::from([
    ("styles".to_string(), styles_path),
    ("common_component".to_string(), common_component_path),
]);
```

### `lib/vocabulary/build.rs`

Add `common_component` to `library_paths`:

```rust
let common_component_path = manifest_dir.join("../../lib/common_component/common_component_lib.slint");
let library_paths = std::collections::HashMap::from([
    ("styles".to_string(), styles_path),
    ("flashcard".to_string(), flashcard_path),
    ("common_component".to_string(), common_component_path),
]);
```

## Patterns and Notes

- `lib/common_component` follows the same libC pattern as `lib/styles` — pure `.slint` files, no Rust crate, no `Cargo.toml`. Each consumer registers the entry file path in its own `build.rs` via `with_library_paths`.
- The path **must point to the entry `.slint` file**, not to the directory (pointing to a directory causes `Access is denied` on Windows).
- Run `cargo build --bin japanese_learn` before committing to confirm zero errors.
