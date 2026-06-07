# Subtask 6.B.1 — Fix Test Matching grid layout and add pagination

**Agent**: slint-developer
**Parent task**: 6.B.1
**Depends on**: none
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `MatchingExerciseView` renders front and back tiles in a balanced two-column grid that fits within the application viewport — front tiles are no longer too wide, and back tiles are visible. The view shows at most 10 card pairs at a time; when the stack has more than 10 cards, a "Next" button advances to the next set of up to 10 (wrapping or disabling at the end is the implementer's call — state it in the commit message). `cargo build` is green.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/components/matching_exercise_view.slint` | modify | constrain tile width, add pagination state (`page-offset`) and a "Next" button, slice `cards` to the visible page |

## Components / Modules

- **`MatchingExerciseView`** — modify only: keep the existing positional-matching logic (`selected-front-index`, `matched-flags`, `matched-count`, `exercise-completed`) operating on the *visible page slice* of `cards`, not the full list. Add a `property <int> page-offset: 0` (or equivalent) that tracks which 10-card window is shown, and a "Next" `CommonBtn` that advances it (e.g. `page-offset = (page-offset + page-size) mod cards.length`, clamped so it never exceeds `cards.length`).

## Patterns and Notes

- Root-cause of the width bug: the front/back `CommonBtn` tiles inherit `width: 100%` from `CommonBtn` inside an unconstrained `HorizontalLayout`/`VerticalLayout` pairing — give each column an explicit `horizontal-stretch: 1` (equal split) or cap tile width with `min(parent.width, <value>)` so both columns fit side by side within the viewport and the back column is visible.
- "About 10 flashcards" — use a named constant (e.g. `property <int> page-size: 10`) rather than a magic number repeated in multiple places.
- Reset `selected-front-index`, `matched-flags`, and `matched-count` when the page changes, since matching state is positional and tied to the currently visible slice.
- Follow the existing `for card[i] in cards: CommonBtn { ... }` two-column pattern already in the file — restructure it, do not replace the matching mechanic.
- This is a pure-UI fix; no Rust changes and no new callbacks are required.
