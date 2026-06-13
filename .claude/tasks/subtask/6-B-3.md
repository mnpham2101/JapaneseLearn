# Subtask 6.B.3 — Fix FlashcardLabel width overlap

**Agent**: slint-developer
**Parent task**: 6.B.3
**Depends on**: none
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `FlashcardLabel` no longer overlaps its container's border in `FlashcardList` — its width is capped to fit within the available space (matching the `min(parent.width, ...)` pattern already used for `LessonStackLabel`/`LessonStackList` rows). `cargo build` is green.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/flashcard/ui/components/flashcard_label.slint` | modify | override the inherited `width: 100%` from `CommonBtn` with a capped width, e.g. `width: min(parent.width, <value>)` |

## Components / Modules

- **`FlashcardLabel`** — modify only: add an explicit `width` binding that caps the label's width to the parent's available width (it currently inherits `width: 100%` from `CommonBtn`, which overflows the centering `HorizontalLayout`/`VerticalLayout` in `FlashcardList`).

## Patterns and Notes

- Follow the same `width: min(parent.width, 200px)` style cap already used for `"＋ Add Lesson"` in `lib/vocabulary/ui/components/lesson_stack_list.slint` (Task 6.S.5b) — pick a sensible max (e.g. `320px`) that keeps the label readable without overflowing.
- Do not change `CommonBtn` itself — the fix is local to `FlashcardLabel` since other `CommonBtn`-derived components rely on the `width: 100%` default within their own constrained containers.
- No Rust changes required.
