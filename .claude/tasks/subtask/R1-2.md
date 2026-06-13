# Subtask R1.2 — Move CommonBtn to lib/common_component

**Agent**: slint-developer  
**Parent task**: R1.2  
**Depends on**: R1.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `CommonBtn` lives in `lib/common_component/components/common_button.slint` and is exported from `common_component_lib.slint`. All consumers — inside `lib/flashcard`, inside `lib/vocabulary`, and in root `ui/` — import `CommonBtn` from `@common_component`. The old file `lib/flashcard/ui/components/common_button.slint` is deleted. `flashcard_lib.slint` no longer exports `CommonBtn`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/common_component/components/common_button.slint` | create | full content of CommonBtn (move from flashcard) |
| `lib/common_component/common_component_lib.slint` | modify | add `export { CommonBtn } from "components/common_button.slint"` |
| `lib/flashcard/ui/components/common_button.slint` | delete | moved to common_component |
| `lib/flashcard/ui/flashcard_lib.slint` | modify | remove `CommonBtn` export |
| `lib/flashcard/ui/components/common_list.slint` | modify | import from `@common_component` instead of relative path |
| `lib/flashcard/ui/components/flashcard_label.slint` | modify | import from `@common_component` |
| `lib/flashcard/ui/components/flashcard_stack.slint` | modify | import from `@common_component` |
| `lib/flashcard/ui/components/flashcard_list.slint` | modify | import from `@common_component` if it imports CommonBtn |
| `lib/flashcard/ui/page/flashcard_manager_view.slint` | modify | import from `@common_component` |
| `lib/flashcard/ui/page/study_session_view.slint` | modify | import from `@common_component` if it imports CommonBtn |
| `lib/vocabulary/ui/components/lesson_stack_label.slint` | modify | `@flashcard` → `@common_component` |
| `lib/vocabulary/ui/components/lesson_stack_list.slint` | modify | `@flashcard` → `@common_component` |
| `lib/vocabulary/ui/components/lesson_detail_view.slint` | modify | `@flashcard` → `@common_component` if it imports CommonBtn |
| `lib/vocabulary/ui/components/lesson_detail_pane.slint` | modify | if it imports CommonBtn |
| `lib/vocabulary/ui/components/matching_exercise_view.slint` | modify | `import { CommonBtn, FlashcardModel } from "@flashcard"` → split: CommonBtn from `@common_component`, FlashcardModel from `@flashcard` |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | `@flashcard` → `@common_component` for CommonBtn |
| `ui/pages/review_page.slint` | modify | `@flashcard` → `@common_component` for CommonBtn |
| `ui/pages/study_page.slint` | modify | if it imports CommonBtn from `@flashcard` |

## Components / Modules

**`CommonBtn`** — unchanged in content; only its location changes from `lib/flashcard/ui/components/common_button.slint` to `lib/common_component/components/common_button.slint`. The component still imports `{ Tokens } from "@styles"` (all consumers register @styles, so this resolves correctly).

## Patterns and Notes

- Files inside `lib/flashcard` that previously used a relative import `import { CommonBtn } from "common_button.slint"` must change to `import { CommonBtn } from "@common_component"`.
- Files outside `lib/flashcard` that used `import { CommonBtn } from "@flashcard"` must change to `import { CommonBtn } from "@common_component"`. If they still need other exports from `@flashcard` (e.g. `FlashcardModel`), keep those imports; only move the `CommonBtn` reference.
- `matching_exercise_view.slint` currently does `import { CommonBtn, FlashcardModel } from "@flashcard"`. Split this into two separate import lines:
  ```slint
  import { CommonBtn } from "@common_component";
  import { FlashcardModel } from "@flashcard";
  ```
- Run `cargo build --bin japanese_learn` and confirm zero errors before committing.
- Run `cargo clippy --bin japanese_learn` to confirm zero warnings.
