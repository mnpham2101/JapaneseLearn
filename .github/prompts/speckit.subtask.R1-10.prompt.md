# Subtask R1.10 — Redesign ReviewPage

**Agent**: slint-developer  
**Parent task**: R1.10  
**Depends on**: R1.5, R1.9  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `ReviewPage` shows vocabulary lessons (from `VocabularyAppLogic.lesson-list`) in the selection list instead of flashcard stacks. Clicking a lesson reveals `TestView` with the corresponding flashcard cards. The page heading shows the selected lesson's name when a test is open. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `ui/pages/review_page.slint` | modify | full redesign as described below |

## New `review_page.slint` Structure

```slint
import { Tokens } from "@styles";
import { CommonBtn } from "@common_component";
import { FlashcardAppLogic } from "@flashcard";
import { VocabularyAppLogic, LessonLabel, TestView } from "@vocabulary";

export component ReviewPage {
    width: 100%;
    height: 100%;

    // Index of the selected lesson (-1 = none).
    property <int> selected-lesson-index: -1;
    // Whether TestView is visible.
    property <bool> show-test-view: false;

    Rectangle {
        width: 100%;
        height: 100%;
        background: Tokens.page-background;

        VerticalLayout {
            padding: Tokens.padding-lg;
            spacing: Tokens.spacing-md;

            // Page heading — shows lesson name when test is open, else "Review Mode".
            Text {
                text: show-test-view && selected-lesson-index >= 0
                    ? VocabularyAppLogic.lesson-list[selected-lesson-index].name
                    : "Review Mode";
                font-size: Tokens.font-size-heading;
                color: Tokens.text-primary;
                horizontal-alignment: center;
            }

            // Lesson selection list — shown when no test is open.
            if !show-test-view: VerticalLayout {
                spacing: Tokens.spacing-sm;

                Text {
                    text: "Select a lesson to practice:";
                    font-size: Tokens.font-size-caption;
                    color: Tokens.text-secondary;
                    horizontal-alignment: center;
                }

                Rectangle {
                    border-color: Tokens.border-color;
                    border-radius: Tokens.border-radius;
                    border-width: Tokens.border-width;

                    VerticalLayout {
                        spacing: Tokens.spacing-sm;
                        padding: Tokens.padding-sm;

                        for lesson[i] in VocabularyAppLogic.lesson-list: LessonLabel {
                            horizontal-stretch: 1;
                            height: 36px;
                            lesson: lesson;
                            checkable: false;
                            clicked => {
                                selected-lesson-index = i;
                                show-test-view = true;
                            }
                        }
                    }
                }
            }

            // TestView — shown when a lesson is selected.
            if show-test-view: TestView {
                cards: selected-lesson-index >= 0
                    && selected-lesson-index < FlashcardAppLogic.flashcard-list.length
                    ? FlashcardAppLogic.flashcard-list[selected-lesson-index].flashcards
                    : [];
                close-clicked => {
                    show-test-view = false;
                    selected-lesson-index = -1;
                }
            }
        }
    }
}
```

## Key Design Decisions

### Lesson → flashcard mapping

`TestView.cards` is fed from `FlashcardAppLogic.flashcard-list[selected-lesson-index].flashcards`. This assumes lesson index `i` maps to flashcard stack index `i` — valid because `Generate Exercises` creates stacks in lesson order. A bounds check (`selected-lesson-index < FlashcardAppLogic.flashcard-list.length`) prevents an out-of-range access when no flashcards have been generated yet.

### No edit buttons

The lesson list in ReviewPage uses `LessonLabel` directly inside a simple `for` loop — no delete button, no create form. This is intentional: ReviewPage is read-only.

### Back navigation

`TestView` fires `close-clicked` when its internal Close button is pressed (after viewing the result). This resets `show-test-view = false` and `selected-lesson-index = -1`, returning to the lesson selection list.

## Patterns and Notes

- `LessonLabel` and `TestView` are now both exported from `@vocabulary`, so the import line `import { ..., LessonLabel, TestView } from "@vocabulary"` covers both.
- The root `build.rs` must have `@vocabulary` registered. Check `build.rs` — if it is not registered, add it:
  ```rust
  let vocabulary_path = manifest_dir.join("lib/vocabulary/ui/vocabulary_lib.slint");
  // add ("vocabulary".to_string(), vocabulary_path) to library_paths
  ```
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
- Run `cargo clippy --bin japanese_learn` for zero warnings.
