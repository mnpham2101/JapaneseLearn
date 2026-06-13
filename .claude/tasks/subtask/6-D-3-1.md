# Subtask 6.D.3.1 — Restore Defaults callback declaration and UI button

**Agent**: slint-developer  
**Parent task**: 6.D.3  
**Depends on**: 6.D.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `VocabularyAppLogic` declares `callback restore-defaults-clicked()`, and the Lesson view (active-view == 0) of `VocabularyPage` shows a "Restore Defaults" `CommonBtn` above `LessonStackList`. Clicking the button fires the callback. No Rust handler is wired yet; the build must pass with the unregistered callback.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/ui/vocabulary_app_logic.slint` | modify | add `callback restore-defaults-clicked()` to `VocabularyAppLogic` |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | add `CommonBtn { text: "Restore Defaults"; }` above `LessonStackList` in the Lesson view |

## Components / Callbacks

- `VocabularyAppLogic` (modify): add one line `callback restore-defaults-clicked();` after the existing `export-vocabulary-clicked` line.
- `VocabularyPage` (modify): in the `if VocabularyAppLogic.selected-lesson-index < 0: HorizontalLayout` block that currently contains only `LessonStackList`, wrap its content in a `VerticalLayout` (or add the button above the existing layout) so the Restore Defaults button appears above the `LessonStackList`. The button must fire `VocabularyAppLogic.restore-defaults-clicked()` when clicked.

### Exact placement

The current Lesson view when no lesson is selected:
```slint
if VocabularyAppLogic.selected-lesson-index < 0: HorizontalLayout {
    alignment: center;
    width: 100%;
    height: 100%;

    LessonStackList {
        width: min(parent.width, 480px);
        height: parent.height;
    }
}
```

Replace with a `VerticalLayout` that stacks the button above the list:
```slint
if VocabularyAppLogic.selected-lesson-index < 0: VerticalLayout {
    width: 100%;
    height: 100%;
    spacing: 8px;
    padding: 8px;

    HorizontalLayout {
        alignment: center;
        CommonBtn {
            text: "Restore Defaults";
            clicked => { VocabularyAppLogic.restore-defaults-clicked(); }
        }
    }

    HorizontalLayout {
        alignment: center;
        vertical-stretch: 1;
        LessonStackList {
            width: min(parent.width, 480px);
            height: parent.height;
        }
    }
}
```

## Patterns and Notes

- The Rust handler for `restore-defaults-clicked` is registered in subtask 6.D.3.2. A dead-code-style compiler warning may appear if slint-build reports an unregistered callback — this is acceptable per `atomic-commit-rule.md` for intermediate chain commits.
- Follow the `*Btn` naming convention — the button itself needs no local name since it has no siblings that reference it.
- Do not change the action-bar area or the Exercise/Flashcard views.
