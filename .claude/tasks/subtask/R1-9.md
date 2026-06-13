# Subtask R1.9 — Create TestView

**Agent**: slint-developer  
**Parent task**: R1.9  
**Depends on**: R1.4, R1.8  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `TestView` exists in `lib/vocabulary/ui/components/test_view.slint` and is exported from `vocabulary_lib.slint`. `TestView` renders a `CommonGrid` with a single active cell — "Matching Test" — that opens `MatchingExerciseView` when clicked. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/test_view.slint` | create | `TestView` component |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | export `TestView` |

## Components / Modules

### `TestView`

Accepts a `cards` property (list of `FlashcardModel`) that is forwarded to `MatchingExerciseView`. Displays a `CommonGrid` with "Matching Test" as the only active cell. Clicking it opens `MatchingExerciseView`. Fires a `close-clicked` callback so the parent (`ReviewPage`) can return to the lesson list after the result view's Close button is tapped.

```slint
import { Tokens } from "@styles";
import { CommonGrid, GridItem } from "@common_component";
import { FlashcardModel } from "@flashcard";
import { MatchingExerciseView } from "matching_exercise_view.slint";

export component TestView inherits Rectangle {
    background: Tokens.page-background;

    // Flashcard cards passed in from ReviewPage (one lesson's flashcard stack).
    in property <[FlashcardModel]> cards;

    // Fired when MatchingExerciseView's Close button resets result-view to false
    // and the user has no test open — parent uses this to navigate back.
    callback close-clicked;

    // Internal state: which test is open.
    property <bool> show-matching: false;

    VerticalLayout {
        spacing: Tokens.spacing-md;
        padding: Tokens.padding-lg;

        // Test type grid — only shown when no test is running.
        if !show-matching: CommonGrid {
            items: [
                { text: "Matching Test", enabled: true, active: true },
            ];
            item-clicked(index) => {
                if (index == 0) {
                    show-matching = true;
                }
            }
        }

        // Matching exercise — shown when Matching Test is selected.
        if show-matching: MatchingExerciseView {
            cards: cards;
            // When user taps Close in the result view, hide the exercise.
            // MatchingExerciseView's Close button resets result-view internally;
            // we watch for the transition back to non-result state via exercise-completed
            // or by detecting Close: wire the result-view reset to trigger close-clicked.
            exercise-completed => {
                // exercise-completed fires when all cards matched; automatically
                // show result view — no further action here.
            }
        }

        // Back button — returns to the test grid from MatchingExerciseView non-result state.
        if show-matching: HorizontalLayout {
            alignment: start;
            CommonBtn {
                width: 100px;
                text: "← Back";
                accessibilityLabel: "Back to test selection";
                clicked => {
                    show-matching = false;
                }
            }
        }
    }
}
```

**Refinement for Close button integration**: `MatchingExerciseView.result-view` is reset to `false` by its own Close button. When `result-view` goes back to `false`, `TestView` should hide the exercise (`show-matching = false`) and fire `close-clicked`. Add a `changed result-view` handler or bind `show-matching` to `!exercise.result-view-was-closed`. The simplest approach: add a `closed` callback to `MatchingExerciseView` that fires when the Close button is clicked, and wire it here:

```slint
// In MatchingExerciseView — add callback:
callback closed;  // fired by Close button in result view

// In TestView:
if show-matching: MatchingExerciseView {
    cards: cards;
    closed => {
        show-matching = false;
        close-clicked();
    }
}
```

Add `callback closed` to `MatchingExerciseView` and fire it from the Close button's `clicked` handler (in addition to resetting `result-view`).

## Patterns and Notes

- `TestView` is intentionally thin — it delegates all matching logic to `MatchingExerciseView`.
- The `cards` property comes from the parent `ReviewPage` and is the flashcard stack corresponding to the selected lesson (by index).
- The `CommonBtn` import is needed for the Back button — import from `@common_component`.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
