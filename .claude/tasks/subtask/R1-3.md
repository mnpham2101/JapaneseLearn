# Subtask R1.3 — Move CommonList to lib/common_component

**Agent**: slint-developer  
**Parent task**: R1.3  
**Depends on**: R1.2  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `CommonList` lives in `lib/common_component/components/common_list.slint` and is exported from `common_component_lib.slint`. All consumers import `CommonList` from `@common_component`. The old file `lib/flashcard/ui/components/common_list.slint` is deleted. `flashcard_lib.slint` no longer exports `CommonList`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/common_component/components/common_list.slint` | create | full content of CommonList (move from flashcard) |
| `lib/common_component/common_component_lib.slint` | modify | add `export { CommonList } from "components/common_list.slint"` |
| `lib/flashcard/ui/components/common_list.slint` | delete | moved to common_component |
| `lib/flashcard/ui/flashcard_lib.slint` | modify | remove `CommonList` export |
| `lib/flashcard/ui/page/flashcard_manager_view.slint` | modify | import `CommonList` from `@common_component` instead of `@flashcard` |

## Components / Modules

**`CommonList`** — unchanged in content; only its location changes. The component already imports `{ CommonBtn } from "common_button.slint"` via relative path. After the move, it lives in the same `lib/common_component/components/` folder alongside `common_button.slint`, so the relative import `import { CommonBtn } from "common_button.slint"` remains valid.

## Patterns and Notes

- `flashcard_manager_view.slint` likely imports `CommonList` from `@flashcard`. After this commit it must import from `@common_component`.
- Check whether any vocabulary components reference `CommonList` via `@flashcard` — if so, update those too.
- After deletion of `common_list.slint` from flashcard, verify `flashcard_lib.slint` no longer attempts to export it.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
