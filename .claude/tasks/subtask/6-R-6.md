# Subtask 6.R.6 — Redesign VocabularyPage

**Agent**: slint-developer
**Parent task**: 6.R.6
**Depends on**: 6.R.3, 6.R.4, 6.R.5
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: `lib/vocabulary/ui/pages/vocabulary_page.slint` is rewritten with a three-tab action bar (Lesson/Exercise/Flashcard) and an Import Lesson action button. View 0 shows `LessonStackList` or `LessonDetailView` depending on selection. View 1 shows Generate Flashcards and Export Vocabulary buttons. View 2 shows `FlashcardManagerView`. `cargo build --bin japanese_learn` passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | rewrite | Full replacement of existing content |

## Layout Structure

```
VocabularyPage (Rectangle, background: page-background)
  VerticalLayout (spacing: 0)

    // Action bar (height: 44px, background: nav-bar-background)
    Rectangle (height: 44px, background: Tokens.nav-bar-background)
      HorizontalLayout (padding-left: 12px, padding-right: 12px, spacing: 8px, alignment: start)
        CommonBtn "Lesson"    (width: 90px, checkable, checked: active-view == 0, clicked => active-view = 0)
        CommonBtn "Exercise"  (width: 90px, checkable, checked: active-view == 1, clicked => active-view = 1)
        CommonBtn "Flashcard" (width: 90px, checkable, checked: active-view == 2, clicked => active-view = 2)
        CommonBtn "Import Lesson" (width: 120px, clicked => VocabularyAppLogic.import-vocabulary-clicked())

    // View 0: Lesson view
    if root.active-view == 0: Rectangle (vertical-stretch: 1, background: page-background)
      // Vertically stacked pattern (VerticalLayout, alignment: end)
      VerticalLayout (alignment: end)
        if VocabularyAppLogic.selected-lesson-index >= 0: LessonDetailView
          close-clicked => { VocabularyAppLogic.selected-lesson-index = -1; }

        if VocabularyAppLogic.selected-lesson-index < 0: HorizontalLayout (alignment: center)
          LessonStackList (width: 400px)

    // View 1: Exercise view
    if root.active-view == 1: Rectangle (vertical-stretch: 1, background: page-background)
      HorizontalLayout (alignment: center, spacing: 12px)
        CommonBtn "Generate Flashcards" (primary: true, clicked => VocabularyAppLogic.generate-exercises-clicked())
        CommonBtn "Export Vocabulary"   (clicked => VocabularyAppLogic.export-vocabulary-clicked())

    // View 2: Flashcard view
    if root.active-view == 2: FlashcardManagerView (vertical-stretch: 1)
```

## Local Properties

- `property <int> active-view: 0` — which tab is shown (0=Lesson, 1=Exercise, 2=Flashcard).

## Imports Required

```slint
import { Tokens } from "@styles";
import { CommonBtn, FlashcardAppLogic } from "@flashcard";
import { VocabularyAppLogic, LessonStackList, LessonDetailView, FlashcardManagerView } from "../vocabulary_lib.slint";
```

Or use relative imports for the components within the same library — either is fine as long as it builds.

## Patterns and Notes

- The action bar mirrors the topic-tab-row pattern in `study_page.slint` (height 44px, `nav-bar-background`).
- The "Import Lesson" button is a direct-action button — it does NOT change `active-view`. It fires `VocabularyAppLogic.import-vocabulary-clicked()` and returns to the current view.
- View 0 uses the **Vertically stacked pattern**: `LessonDetailView` slides up when a lesson is selected; `LessonStackList` is shown when no lesson is selected. The `VerticalLayout` has `alignment: LayoutAlignment.end`.
- `LessonDetailView.close-clicked` must reset `VocabularyAppLogic.selected-lesson-index = -1`.
- The existing `LessonList` and `LessonDetailPane` imports are removed from this file (they are now internal to `LessonStackList` and `LessonDetailView` respectively). The `LessonList` and other components still exist in the library — they just are not used directly in `VocabularyPage` any more.
- `FlashcardManagerView` already lives in `lib/vocabulary` (after 6.R.5), so importing it does not create a circular dependency.
