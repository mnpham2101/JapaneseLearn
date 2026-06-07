# Subtask 6.T1.2 — Point `styles.slint` at `themes/theme_default.slint`

**Agent**: slint-developer
**Parent task**: 6.T1
**Depends on**: 6.T1.1
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `lib/styles/styles.slint` re-exports `Tokens` from `themes/theme_default.slint` (not from a top-level `tokens.slint`). The build is green and the app shows zero visual change. If subtask 6.T1.1 already updated this import path as part of its commit, this subtask is a no-op — confirm the current state of `styles.slint` before making any change, and skip committing if there is nothing left to do (report that 6.T1.1 already covered it).

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/styles/styles.slint` | modify | change `export { Tokens } from "tokens.slint";` → `export { Tokens } from "themes/theme_default.slint";` |

## Components / Modules

- **`styles.slint`** — the entry re-export file for the `@styles` library. Only the `Tokens` re-export line's source path changes; the `Animations` re-export is untouched.

## Functions / Callbacks

None — this is a one-line import path change.

## Patterns and Notes

- Switching the active theme in the future is exactly this kind of one-line string replacement — this commit is the template for that swap.
- Verify zero visual regression by confirming `cargo build` succeeds and (optionally) `cargo run --bin japanese_learn` renders the same palette as before.
