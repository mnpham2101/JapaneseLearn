# Subtask R1.7 — Update VocabularyPage

**Agent**: slint-developer  
**Parent task**: R1.7  
**Depends on**: R1.6  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `VocabularyPage` has two action-bar tabs — Lesson (view=0) and Exercise (view=1) — plus the Import Lesson action button. The Flashcard tab and its associated view (active-view=2) are removed. The Exercise view now renders `ExerciseView` (which contains the Generate/Export buttons and `ExerciseGrid`). Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | remove Flashcard tab button; remove view-2 block; replace Exercise view content with `ExerciseView` |

## What Changes in `vocabulary_page.slint`

### Action bar — remove Flashcard tab

Remove the third `CommonBtn` in the action bar (the one that sets `active-view = 2`). The bar now has exactly two tab buttons (Lesson, Exercise) plus the Import Lesson action button.

### View 1 (Exercise) — replace content with ExerciseView

**Before:**
```slint
if VocabularyAppLogic.active-view == 1: Rectangle {
    vertical-stretch: 1;
    background: Tokens.page-background;

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        CommonBtn {
            text: "Generate Flashcards";
            primary: true;
            clicked => { VocabularyAppLogic.generate-exercises-clicked(); }
        }

        CommonBtn {
            text: "Export Vocabulary";
            clicked => { VocabularyAppLogic.export-vocabulary-clicked(); }
        }
    }
}
```

**After:**
```slint
if VocabularyAppLogic.active-view == 1: ExerciseView {
    vertical-stretch: 1;
}
```

### View 2 — remove entirely

Delete the `if VocabularyAppLogic.active-view == 2: FlashcardManagerView { ... }` block. `FlashcardManagerView` is now accessed via `ExerciseView` → `ExerciseGrid`.

### Imports — clean up

- Remove `import { FlashcardManagerView } from "@flashcard"` (no longer used directly by VocabularyPage).
- Add `import { ExerciseView } from "../components/exercise_view.slint"`.
- Keep `import { CommonBtn } from "@common_component"` (still used for action bar buttons and Lesson view).

## Patterns and Notes

- `VocabularyAppLogic.active-view` still works with only two valid values (0 and 1) after this change. No Rust change needed — the Rust init handler sets `active_view` but only ever sets it to 0 (Lesson) or 2 (Flashcard from generate notification). After R1.7, the notification handler in R1.6 sets `show-flashcard-manager = true` inside `ExerciseView` and switches to `active-view = 1`. If the Rust code currently sets `active_view = 2`, that becomes a no-op (view 2 no longer exists in Slint) — check `lib/vocabulary/src/lib.rs` and update to `active_view = 1` if needed. That Rust change is in scope for this subtask since it is a direct consequence of removing view 2.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
- Run `cargo clippy --bin japanese_learn` for zero warnings.
