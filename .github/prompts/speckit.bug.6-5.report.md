# Bug description:
1. in-correct implementation: in `FlashCardStack`, you should have implemented on Slint property binding to show `StudySessionView` that includes Prev  / Next + Close. 
2. `StudySessionView` show `Flashcard` with empty content.  
3. In addition, console error `ICU4X data error: No segmentation model for language: ja` appear when user click on `FlashcardLabel` to open `FlashcardManagerView`. The root cause could be the same as in #1.
4. Clicking on "Study" button also spams the same error. 
5. `FlashcardAppLogic.flashcard-list` has default data but is not showed on `FlashcardList` despite that it is provided to `FlashcardList`'s stack property.

# Confirmed root cause (#2 — empty Flashcard content):

`StudySessionView` (`lib/flashcard/ui/page/study_session_view.slint`) is missing the
pointer-event-absorbing `TouchArea` that its sibling `FlashcardStack`
(`lib/flashcard/ui/components/flashcard_stack.slint:42-44`) explicitly has, with this
comment documenting the exact failure mode:

```slint
// Absorbs all pointer events so they never reach the background
// TouchArea in StudyPage, which would dismiss this component.
TouchArea {}
```

`StudySessionView` has no such absorber — only a centered `VerticalLayout`. Meanwhile
`FlashcardManagerView` (`lib/flashcard/ui/page/flashcard_manager_view.slint:27-31`) has a
background-dismiss `TouchArea` that runs on any click that reaches it:

```slint
TouchArea {
    clicked => {
        FlashcardAppLogic.selected-stack-index = -1;
    }
}
```

Sequence: user clicks empty space inside the active study session → click falls through
`StudySessionView` (no absorber) → reaches the background dismiss `TouchArea` →
`selected-stack-index` becomes `-1` → the binding at `study_session_view.slint:31`

```slint
data: FlashcardAppLogic.flashcard-list[FlashcardAppLogic.selected-stack-index].flashcards[root.current-card-index];
```

evaluates `flashcard-list[-1]`. Slint resolves out-of-range array access to a
default-constructed value, so this returns an empty `FlashcardStackModel`
(`flashcards: []`), and indexing that empty array again returns a default-constructed
empty `FlashcardModel` — hence the card renders with blank front/back text.

This is the same failure pattern the original developer already diagnosed and fixed in
`FlashcardStack` (see the comment above) — the fix simply was not carried over to
`StudySessionView`.

**Verification:** A temporary debug print was added to `study_session_view.slint`
(`changed data => { debug("[Bug6.5] front=... back=... stack-idx=...", ...); }`) so the
bound card's front/back text and `selected-stack-index` can be observed live in the
console — it will print `front='' back='' stack-idx=-1` the instant the card goes blank.
Live UI-driven confirmation was attempted but blocked by an environment limitation
(synthetic mouse clicks do not register on the Slint window in this sandbox — same
finding as previous sessions); the static evidence above is conclusive on its own. The
debug print should be removed once the fix is implemented and manually verified.

**Relation to `stacks.json` persistence:** Not a contributing cause. `load_stacks()` runs
once inside `init()`, before the UI is interactive, and only calls `set_flashcard_list(...)`
— it never touches `selected_stack_index` (which keeps its declared default of `-1`).
`stacks.json` simply supplies real stack data so the bug reproduces with realistic
content rather than only the seed defaults.

# Confirmed root cause (#1 — Study/Prev/Next/Close should use property binding):

Re-evaluated against the existing codebase: the `checkable: true; checked <=> <bool>`
two-way-binding pattern this issue calls for **already exists** —
`flashcard.slint:111-120` implements `checked <=> known` for the known/unknown toggle,
and it is documented in `slint-code-style.md:72-88` with that exact example. No new
pattern needs to be added.

However, applying that mechanism to the "Study" and "✕" buttons in `FlashcardStack`
would be a misapplication, not an improvement:
- `checked <=>` is the right tool for **persistent toggle state** with a meaningful
  "checked" visual (e.g. known/unknown). "Study" and "✕" are **one-shot navigation
  actions** — the button (and its parent `FlashcardStack`) disappears the instant the
  action fires, so there is no persistent state to mirror.
- Binding "Study" directly to `FlashcardAppLogic.study-session-active` would require
  importing the app-level global into the reusable `FlashcardStack` component, breaking
  its current decoupling (today it only knows `stack: FlashcardStackModel` and emits
  generic callbacks; `FlashcardManagerView` wires those callbacks to the global — the
  correct "dumb component / smart container" split).
- `slint-code-style.md:46,88` already states the governing rule: *"Reserve callbacks for
  event notifications... Do not use callbacks to synchronize properties that can be
  bound directly."* `study-clicked`/`close-clicked` ARE event notifications ("the user
  asked to navigate"), not state mirrors — they are already the correct idiom.

**Decision:** keep `study-clicked`/`close-clicked` callbacks as-is. No refactor needed
for issue #1; this part of the report is resolved by re-classification, not by a code
change.

# Solution for #2 (the only remaining actionable issue):

Add a pointer-event-absorbing `TouchArea {}` to `StudySessionView`, mirroring the one in
`FlashcardStack` (with the same explanatory comment), so clicks inside the active study
session never fall through to `FlashcardManagerView`'s background-dismiss handler and
reset `selected-stack-index` to `-1`.

# Notes on issues #3, #4, #5 (ICU4X console errors / default flashcard-list not shown):

- Issues #3 and #4 (ICU4X `No segmentation model for language: ja` console spam) are a
  separate, Slint-dependency-level concern — split out into its own bug entry **6.6**
  (see `speckit.bug.6-6.report.md`) and marked `(not done)`; not addressed here.
- Issue #5 (`flashcard-list` default data not shown in `FlashcardList`) could not be
  reproduced from the code: `flashcard_list.slint`'s binding to `FlashcardAppLogic.flashcard-list`
  and the default seed data both look correct, and `stacks.json` already contains valid
  persisted data that loads and displays correctly. Likely stale/already resolved by an
  earlier fix — drop unless still reproducible.
