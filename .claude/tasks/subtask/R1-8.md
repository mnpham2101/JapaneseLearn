# Subtask R1.8 — Redesign MatchingExerciseView

**Agent**: slint-developer  
**Parent task**: R1.8  
**Depends on**: R1.3  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `MatchingExerciseView` has these new behaviors:
1. **Page size 5** (was 10) — at most 5 card pairs visible per page.
2. **Submit button** — shown in the main (non-result) view; user submits only after navigating all pages.
3. **No correctness revealed until Submit** — matched pairs change to a neutral "selected" color, not green/red.
4. **Result view** — toggled by `result-view` boolean property. Shows correct/incorrect coloring (green/pink border), matched-count/total text, and a Close button. No Submit button in result view.
5. The matched-count/total `Text` previously in the header is removed from the main view and moved to the result view only.
6. **No new Rust logic** — all changes are pure Slint.

Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/matching_exercise_view.slint` | modify | all changes described below |

## Design Changes in Detail

### New properties

```slint
// When true, shows the result view (correct/incorrect coloring, Close button).
// Set to true by the Submit button; reset to false by the Close button.
in-out property <bool> result-view: false;

// Per-tile "correct match" flags — set at Submit time.
// true = this positional pair was correctly matched.
property <[bool]> correct-flags: [false, false, ...];  // same length as matched-flags
```

### Remove from header

Delete the `Text { text: matched-count + " / " + cards.length; ... }` from the header row.

### Tile coloring

**Main view (result-view == false):**
- Matched tiles: `primary: matched-flags[i]` (blue/primary color as before — neutral, not green/red).
- Selected front tile: `checked: (selected-front-index == i) && !matched-flags[i]`.

**Result view (result-view == true):**
- Correctly matched tiles: light green border — `border-color: #a8d8a8; border-width: 2px`.
- Incorrectly matched tiles (matched but wrong position not applicable here since matching IS positional) — for tiles that were NOT matched at all when Submit was pressed: pink border — `border-color: #f4a0a0; border-width: 2px`.
- All tiles disabled in result view.

### Submit button

Shown when `!result-view` and at least one match has been made (or always show it — keep it always visible in main view for simplicity):

```slint
if !result-view: HorizontalLayout {
    alignment: center;
    spacing: Tokens.spacing-md;

    CommonBtn {
        width: min(parent.width, 160px);
        text: "Submit";
        primary: true;
        accessibilityLabel: "Submit matching exercise";
        clicked => {
            root.result-view = true;
            // Mark correct flags: a pair is "correct" if matched-flags[i] == true
            // (in this positional matching scheme, any matched pair is correct by definition)
            // Unmatched pairs have correct-flags[i] = false (shown with pink border)
        }
    }
}
```

### Result view

Shown when `result-view == true`:

```slint
if result-view: HorizontalLayout {
    alignment: center;
    spacing: Tokens.spacing-md;

    Text {
        text: matched-count + " / " + cards.length;
        font-size: Tokens.font-size-caption;
        color: Tokens.text-secondary;
        vertical-alignment: center;
    }

    CommonBtn {
        width: min(parent.width, 120px);
        text: "Close";
        accessibilityLabel: "Close result view";
        clicked => {
            result-view = false;
            // Reset state for reuse
            matched-count = 0;
            selected-front-index = -1;
            page-offset = 0;
            matched-flags = [false, false, ...];
        }
    }
}
```

### Tile border coloring in result view

Inside each tile's `for` loop, conditionally add border styling:

```slint
for card[i] in cards: CommonBtn {
    visible: i >= page-offset && i < page-offset + visible-count;
    // ...existing properties...
    border-color: result-view
        ? (matched-flags[i] ? #a8d8a8 : #f4a0a0)
        : Tokens.border-color;
    border-width: result-view ? 2px : 0px;
    enabled: !result-view && !matched-flags[i];
    // ...existing clicked handler, disabled in result-view...
}
```

### `VocabularyTestWindow` in vocabulary_lib.slint

Update `MatchingExerciseTestWindow` to expose the new `result-view` property so tests can interact with it.

## Patterns and Notes

- In the positional matching scheme used here, a "correctly matched" pair is one where the user matched front[i] with back[i]. Since the current implementation only allows matching same-index pairs (correct by definition), ALL matched pairs are correct. Unmatched pairs (not matched when Submit is pressed) get the pink border.
- The `correct-flags` property is therefore: `correct-flags[i] = matched-flags[i]` — i.e. every matched pair is correct. No separate tracking needed; simplify to use `matched-flags` directly in the result view coloring.
- `exercise-completed` callback: remove it or keep it (it fires when all cards are matched). Since the Submit button now controls the flow, the callback is less critical but can be retained for backward compatibility.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
