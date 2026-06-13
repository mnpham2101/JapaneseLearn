# Bug description:

Console error `ICU4X data error: No segmentation model for language: ja` appears
repeatedly while the app renders Japanese text — e.g. when opening
`FlashcardManagerView` (clicking a `FlashcardLabel`) and when clicking the "Study"
button to enter a study session. Originally noted as items #3 and #4 in
`speckit.bug.6-5.report.md` ("The root cause could be the same as in #1" /
"Clicking on 'Study' button also spams the same error"). Split out here because the
root cause is unrelated to the `selected-stack-index` binding issue tracked in 6.5 —
it is a Slint/ICU4X text-segmentation dependency concern, not application logic.

# Root cause (preliminary — not yet confirmed with the user):

ICU4X (Internationalization Components for Unicode) is the text-segmentation backend
Slint uses internally for line/word breaking and shaping. The error indicates the
runtime has no segmentation data for the `ja` (Japanese) locale. This is triggered
whenever Slint needs to segment Japanese text for layout — most visibly on `Text`
elements with `wrap: word-wrap` (e.g. `flashcard.slint:89`, the back-face explanation
text) and on `TextInput`/`Text` elements rendering Japanese strings (stack names, card
fields, "Study"-session content).

There is **no direct ICU4X dependency** in this project's `Cargo.toml` — it is a
transitive dependency pulled in by Slint itself. This makes it a platform/dependency
issue rather than something fixable by changing application code, and any fix would
likely involve a Slint version bump, a Slint feature flag, or bundling/configuring an
ICU4X data provider — all of which need research and explicit user approval before any
change is attempted (per the bugfix-tasks `CRITICAL bug / dependency issue` variant).

# Status:

**(not done)** — documented only. Per explicit instruction, this bug is **not** being
worked on in this session; it is recorded here so it can be picked up and investigated
(Slint version/feature research, ICU4X data-provider options) in a future pass.
