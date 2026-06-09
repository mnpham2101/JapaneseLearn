# Subtask R1.5 — Rename LessonStackLabel → LessonLabel

**Agent**: slint-developer  
**Parent task**: R1.5  
**Depends on**: R1.3  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, the component previously named `LessonStackLabel` is renamed to `LessonLabel` everywhere: the file is renamed, the component declaration is updated, all import sites and usages in vocabulary components are updated, and `vocabulary_lib.slint` exports `LessonLabel` instead of `LessonStackLabel`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/lesson_stack_label.slint` | rename + modify | rename file to `lesson_label.slint`; rename component from `LessonStackLabel` to `LessonLabel` |
| `lib/vocabulary/ui/components/lesson_stack_list.slint` | modify | update import path and usage: `LessonStackLabel` → `LessonLabel` |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | update export: `LessonStackLabel` → `LessonLabel`, update file path reference |

## Components / Modules

**`LessonLabel`** (renamed from `LessonStackLabel`) — no behavioral change. Same properties (`lesson`, `text`, inherited `CommonBtn` states). Only the component name and file name change.

After the rename, `lesson_label.slint` content:

```slint
import { CommonBtn } from "@common_component";
import { VocabularyLessonModel } from "../vocabulary_app_logic.slint";

export component LessonLabel inherits CommonBtn {
    in property <VocabularyLessonModel> lesson: { name: "", words: [] };
    text: lesson.name;
}
```

Note: the import also changes from `@flashcard` to `@common_component` (consistent with R1.2).

## Patterns and Notes

- This is a pure rename — no behavioral change, no new properties.
- `lesson_stack_list.slint` currently imports `LessonStackLabel` from `"lesson_stack_label.slint"`. After the rename, the import becomes `import { LessonLabel } from "lesson_label.slint"` and all usages of `LessonStackLabel` in the component body become `LessonLabel`.
- Check whether any other vocabulary or root UI files reference `LessonStackLabel` — update those too.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
