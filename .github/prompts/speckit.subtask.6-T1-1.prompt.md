# Subtask 6.T1.1 — Move `Tokens` global into `themes/theme_default.slint`

**Agent**: slint-developer
**Parent task**: 6.T1
**Depends on**: none (first in chain)
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `lib/styles/themes/theme_default.slint` exports the `Tokens` global with the exact same property names, types, and values that previously lived in `lib/styles/tokens.slint`. The old `lib/styles/tokens.slint` file no longer exists. The build is green and the running app shows zero visual change — this is a pure file relocation, not a content change.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/styles/themes/theme_default.slint` | create | new file containing `export global Tokens { ... }` — copy the full contents of `lib/styles/tokens.slint` verbatim (all ~50 properties: border, card, text, spacing, font-size, button states, known/unknown indicators, page/nav backgrounds, input fields, stack labels, drag-target highlight, bar-level distinction) |
| `lib/styles/tokens.slint` | delete | removed — its contents now live in `themes/theme_default.slint` |

## Components / Modules

- **`Tokens` global** (Slint) — the single source of truth for design tokens. Its definition moves location only; no property is added, removed, or renamed.

## Functions / Callbacks

None — this is a pure relocation of a Slint global definition.

## Patterns and Notes

- Read `lib/styles/tokens.slint` first to get the authoritative property list and exact values — copy verbatim, do not retype from memory.
- This commit will leave `lib/styles/styles.slint`'s `export { Tokens } from "tokens.slint";` line referencing a now-deleted file — that line is fixed in the very next subtask (6.T1.2). It is acceptable for the build to be temporarily broken between these two commits ONLY if you commit both in immediate sequence; otherwise, prefer to make this single commit self-consistent by also updating the import path here. **Recommended**: update the import path in `styles.slint` as part of THIS commit (a one-line path change is trivial and keeps every commit green) — but if you do so, mention it in the commit message as part of the move. If you choose to keep 6.T1.1 and 6.T1.2 strictly separate, ensure `cargo build` is green at the end of 6.T1.1 by including the path fix here regardless (the atomic-commit-rule requires every commit to build).
- Folder `lib/styles/themes/` does not yet exist — create it.
