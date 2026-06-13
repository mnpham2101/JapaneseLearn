# Subtask R1.6 — Create ExerciseView and ExerciseGrid

**Agent**: slint-developer  
**Parent task**: R1.6  
**Depends on**: R1.4  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `ExerciseView` and `ExerciseGrid` exist in `lib/vocabulary/ui/components/` and are exported from `vocabulary_lib.slint`. `ExerciseView` contains a row with "Generate Exercises" and "Export Vocabulary" action buttons. `ExerciseGrid` contains a `CommonGrid` configured with one active cell — "Flashcard" — that opens `FlashcardManagerView` when clicked. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/exercise_grid.slint` | create | `ExerciseGrid` component |
| `lib/vocabulary/ui/components/exercise_view.slint` | create | `ExerciseView` component |
| `lib/vocabulary/ui/vocabulary_lib.slint` | modify | export `ExerciseView` and `ExerciseGrid` |

## Components / Modules

### `ExerciseGrid`

Wraps `CommonGrid` with a fixed `items` list where "Flashcard" is the one active entry (index 0). Fires `flashcard-clicked` callback when the Flashcard cell is tapped. Lives at `lib/vocabulary/ui/components/exercise_grid.slint`.

```slint
import { Tokens } from "@styles";
import { CommonGrid, GridItem } from "@common_component";

export component ExerciseGrid inherits Rectangle {
    callback flashcard-clicked;

    CommonGrid {
        items: [
            { text: "Flashcard", enabled: true, active: true },
        ];
        item-clicked(index) => {
            if (index == 0) {
                flashcard-clicked();
            }
        }
    }
}
```

### `ExerciseView`

Contains:
1. A row with "Generate Exercises" button (fires `VocabularyAppLogic.generate-exercises-clicked()`) and "Export Vocabulary" button (fires `VocabularyAppLogic.export-vocabulary-clicked()`).
2. An `ExerciseGrid` below; its `flashcard-clicked` callback sets `show-flashcard-manager = true`.
3. A `FlashcardManagerView` overlay (shown when `show-flashcard-manager == true`).

```slint
import { Tokens } from "@styles";
import { CommonBtn } from "@common_component";
import { VocabularyAppLogic } from "../vocabulary_app_logic.slint";
import { FlashcardManagerView } from "@flashcard";
import { ExerciseGrid } from "exercise_grid.slint";

export component ExerciseView inherits Rectangle {
    background: Tokens.page-background;

    // When true, FlashcardManagerView overlays the grid.
    property <bool> show-flashcard-manager: false;

    VerticalLayout {
        spacing: Tokens.spacing-md;
        padding: Tokens.padding-lg;

        // Action row — generate and export
        HorizontalLayout {
            alignment: center;
            spacing: Tokens.spacing-md;

            CommonBtn {
                text: "Generate Exercises";
                primary: true;
                clicked => {
                    VocabularyAppLogic.generate-exercises-clicked();
                }
            }

            CommonBtn {
                text: "Export Vocabulary";
                clicked => {
                    VocabularyAppLogic.export-vocabulary-clicked();
                }
            }
        }

        // Exercise type grid
        ExerciseGrid {
            flashcard-clicked => {
                show-flashcard-manager = true;
            }
        }
    }

    // Flashcard manager overlay — shown when a grid cell is activated
    if show-flashcard-manager: FlashcardManagerView {
        width: 100%;
        height: 100%;
    }
}
```

## Patterns and Notes

- `ExerciseView` hosts `FlashcardManagerView` directly (replaces the former active-view=2 path in `VocabularyPage`). FlashcardManagerView has its own background `TouchArea` dismiss; the user returns to `ExerciseView` by clicking the background or navigating away within FlashcardManagerView.
- The `generate-exercises-clicked` and `export-vocabulary-clicked` callbacks were previously wired in `VocabularyPage`'s Exercise view. Moving them into `ExerciseView` keeps the component self-contained.
- `ExerciseGrid` is intentionally separate from `ExerciseView` so `CommonGrid` remains reusable for `TestView` (R1.9) without coupling to vocabulary-specific callbacks.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
