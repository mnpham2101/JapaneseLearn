# Subtask 6.S.5 — Fix layout bugs and responsive widths

**Agent**: slint-developer  
**Parent task**: 6.S.5  
**Depends on**: 6.S.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit: (a) the Confirm/Cancel form in `LessonStackList` no longer overlaps the action bar when "Add Lesson" is clicked; (b) the "Add Lesson" button has an appropriate max width; (c) fixed pixel widths are replaced with responsive constraints in `vocabulary_page.slint` and `study_session_view.slint`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | Fix Lesson view layout; remove fixed 400px width on LessonStackList |
| `lib/vocabulary/ui/components/lesson_stack_list.slint` | modify | Cap "Add Lesson" button width; ensure LessonStackList fills its parent height |
| `lib/flashcard/ui/components/study_session_view.slint` | modify | Remove fixed 360px width on Flashcard |

---

## Bug: Confirm/Cancel overlap with action bar

**Root cause**: In `vocabulary_page.slint`, the Lesson view (active-view == 0) uses:

```slint
VerticalLayout {
    alignment: LayoutAlignment.end;

    if selected-lesson-index < 0: HorizontalLayout {
        alignment: center;
        LessonStackList { width: 400px; }
    }
}
```

`LessonStackList` inherits `VerticalLayout` with `alignment: end`. Since it has no explicit height, it is sized by its preferred content height. When the create-form opens (+64px), the component grows downward but alignment-end can push it below the view bounds, causing visual overlap with the action bar.

**Fix**: Remove the intermediate `VerticalLayout` wrapper in the Lesson view. Give the `HorizontalLayout` full height and make `LessonStackList` fill it:

```slint
// ── View 0: Lesson ────────────────────────────────────────────────────
if root.active-view == 0: Rectangle {
    vertical-stretch: 1;
    background: Tokens.page-background;

    if VocabularyAppLogic.selected-lesson-index >= 0: LessonDetailView {
        width: 100%;
        height: 100%;
        close-clicked => {
            VocabularyAppLogic.selected-lesson-index = -1;
        }
    }

    if VocabularyAppLogic.selected-lesson-index < 0: HorizontalLayout {
        alignment: center;
        height: 100%;

        LessonStackList {
            width: min(parent.width, 480px);
            height: parent.height;
        }
    }
}
```

With `height: parent.height` on `LessonStackList`, its internal `Flickable` (vertical-stretch: 1) properly fills the space above the form/button, and the form appears at the bottom without overflowing.

---

## Fix: "Add Lesson" button max width

In `lesson_stack_list.slint`, the "Add Lesson" button currently uses `width: 100%` (CommonBtn default). Wrap it in a centering `HorizontalLayout` and cap the width:

```slint
// "＋ Add Lesson" button — hidden while the form is open.
if !root.show-create-form: HorizontalLayout {
    alignment: center;
    CommonBtn {
        width: min(parent.width, 200px);
        height: 36px;
        text: "＋ Add Lesson";
        primary: true;
        accessibilityLabel: "Open add lesson form";
        clicked => {
            root.show-create-form = true;
        }
    }
}
```

The `min(parent.width, 200px)` ensures the button never exceeds 200px while still shrinking on very narrow windows.

---

## Fix: StudySessionView fixed Flashcard width

In `lib/flashcard/ui/components/study_session_view.slint`, the `Flashcard` inside the `HorizontalLayout` has a fixed `width: 360px`. Replace with a responsive constraint:

```slint
HorizontalLayout {
    alignment: center;
    session-card := Flashcard {
        width: min(parent.width - 32px, 480px);
        // ...rest unchanged
    }
}
```

`min(parent.width - 32px, 480px)` allows the card to fill narrow windows (with 16px padding each side) while capping at 480px on wide screens.

---

## Patterns and Notes

- Do not add `vertical-stretch: 1` to `LessonDetailView` — it already expands to fill its parent when `width: 100%; height: 100%` are set.
- The `HorizontalLayout { alignment: center; height: 100%; }` in the Lesson view is semantically an alignment wrapper, not a stacked-pattern parent. This is acceptable because it does not own stacking logic — `LessonStackList` owns its own stack pattern internally.
- Verify that the create-lesson form slides up correctly without overlap after the fix by running the app and clicking "Add Lesson" several times.
