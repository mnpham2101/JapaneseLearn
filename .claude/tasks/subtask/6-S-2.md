# Subtask 6.S.2 — CommonBtn: checked-bg property, press feedback, hide disabled from layout

**Agent**: slint-developer  
**Parent task**: 6.S.2  
**Depends on**: 6.S.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `CommonBtn` supports per-instance `checked-bg` overrides (so VocabularyPage action-bar tabs can use a distinct active color), shows a distinct pressed color on click-and-hold (using `btn-pressed-bg`), and completely hides disabled buttons from the layout (using `visible: enabled`). The VocabularyPage action-bar tab buttons (`Lesson`/`Exercise`/`Flashcard`) have `checked-bg: Tokens.action-tab-btn-active-bg`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed

| File | Action | What changes |
|---|---|---|
| `lib/flashcard/ui/components/common_button.slint` | modify | Add `checked-bg`; update background expression; add `visible: enabled` |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | Set `checked-bg: Tokens.action-tab-btn-active-bg` on the 3 action-bar tab buttons |

## CommonBtn Changes

### Add `checked-bg` in-property

```slint
// Color used for the button background when checked == true.
// Override at usage site for action-bar tabs (distinct from topic tabs).
in property <color> checked-bg: Tokens.nav-background;
```

Place this after the `checked` property declaration.

### Update background expression

Replace the existing background ternary to include pressed state and use `checked-bg`:

```slint
background: !enabled ? Tokens.btn-disabled-bg
    : (ta.pressed ? Tokens.btn-pressed-bg
    : (checked ? checked-bg
    : (primary ? Tokens.btn-primary-bg
    : Tokens.btn-default-bg)));
```

Remove the separate `opacity` line that handled pressed feedback (or keep it at 1.0 since background now handles it). If opacity is kept, set it to a constant `1.0` to avoid double-dimming.

### Hide disabled buttons from layout

Add at the top-level Rectangle of CommonBtn:
```slint
visible: enabled;
```

This completely removes the button from layout flow when `enabled: false`, which is the specified behavior for disabled buttons (they must not occupy space).

> **Design note**: Navigation buttons in `StudySessionView` (Previous/Next) use `enabled` for boundary clamping. With `visible: enabled`, they disappear at the first/last card. This is intentional per the requirements ("Disable buttons should not appear on layout").

## VocabularyPage Changes

In `lib/vocabulary/ui/pages/vocabulary_page.slint`, set `checked-bg` on the three action-bar tab buttons:

```slint
CommonBtn {
    width: 90px;
    text: "Lesson";
    checkable: true;
    checked: root.active-view == 0;
    checked-bg: Tokens.action-tab-btn-active-bg;
    clicked => { root.active-view = 0; }
}
// ...same for Exercise and Flashcard buttons
```

Do **not** set `checked-bg` on the "Import Lesson" button — it is a direct-action button, not a tab.

## Patterns and Notes

- The `text-on-dark` token is the text color for checked/primary buttons. After 6.S.1 sets `text-on-dark: #FEFAE0`, the text on dark-brown (`nav-background`) and tan (`action-tab-btn-active-bg`) backgrounds will both appear in cream. Verify contrast is acceptable.
- StudyPage topic tab buttons (`Vocabulary`/`Grammar`/`Reading`) use the default `checked-bg: Tokens.nav-background` (#251504) — no change needed there.
- Verify `cargo build --bin japanese_learn` and `cargo test -p flashcard-tests` pass.
