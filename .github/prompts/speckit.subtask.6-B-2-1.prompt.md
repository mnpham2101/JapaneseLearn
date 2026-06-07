# Subtask 6.B.2.1 — Add notification + tab-switch plumbing to VocabularyAppLogic

**Agent**: slint-developer
**Parent task**: 6.B.2
**Depends on**: none
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `VocabularyAppLogic` exposes an `in-out property <string> generation-notification` (the message to display, empty string = hidden) and `active-view` is promoted from a local `VocabularyPage` property to `in-out property <int> active-view` on `VocabularyAppLogic` (so Rust can switch tabs after generation completes). `VocabularyPage` binds its action-bar tabs and view-switch `if` blocks to `VocabularyAppLogic.active-view` instead of a local `root.active-view`, and renders a dismissible banner `Text`/`Rectangle` showing `VocabularyAppLogic.generation-notification` when it is non-empty (clicking it, or a small "✕", clears it back to `""`). `cargo build` is green — the property exists but nothing sets `generation-notification` yet (that is wired in 6.B.2.2).

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add `in-out property <string> generation-notification: ""`; add `in-out property <int> active-view: 0` |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | replace local `property <int> active-view` with bindings to `VocabularyAppLogic.active-view`; add a banner that shows/clears `generation-notification` |

## Components / Modules

- **`VocabularyAppLogic`** — add two properties: `generation-notification` (string, default `""`) and `active-view` (int, default `0`). No new callbacks.
- **`VocabularyPage`** — remove its local `property <int> active-view: 0`; replace every `root.active-view` read/write (tab `checked` bindings, `clicked` handlers, the three `if root.active-view == N` blocks) with `VocabularyAppLogic.active-view`. Add a banner — a `Rectangle`/`Text` row shown `if VocabularyAppLogic.generation-notification != ""` displaying the message, with a `TouchArea` or small `CommonBtn { text: "✕" }` that sets `VocabularyAppLogic.generation-notification = ""`.

## Patterns and Notes

- This subtask only adds the declarations and UI plumbing — it must build and run with the banner simply never appearing (since nothing sets the string yet). That is an acceptable intermediate state per `atomic-commit-rule.md`.
- Follow the existing `in-out property <[VocabularyLessonModel]> lesson-list` / `in-out property <int> selected-lesson-index` pattern already in `vocabulary_app_logic.slint` for declaring the new in-out properties.
- Style the banner using existing `Tokens` (e.g. `Tokens.btn-primary-bg` background, `Tokens.text-on-dark` text) — do not add new design tokens; this is out of scope.
- Place the banner so it is visible regardless of `active-view` (e.g. above the action bar or as an overlay row in the root `VerticalLayout`), since the notification must be visible at the moment the view switches to the Flashcard tab.
