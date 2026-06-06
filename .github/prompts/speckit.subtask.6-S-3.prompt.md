# Subtask 6.S.3 — Flashcard flip animation using width-compression trick

**Agent**: slint-developer  
**Parent task**: 6.S.3  
**Depends on**: 6.S.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, tapping a `Flashcard` in the study session triggers a visible card-flip animation (front collapses edge-on, back expands from zero) instead of an instant content swap. All flip timing lives in `Animations` tokens. The external `show-back` property interface is unchanged so `StudySessionView` continues to work without modification. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed

| File | Action | What changes |
|---|---|---|
| `lib/styles/animations.slint` | modify | Add `flip-delay` token |
| `lib/flashcard/ui/components/flashcard.slint` | modify | Replace `if show-back` with width-compression animation |

## animations.slint Change

Add one token alongside the existing `flip-duration`:

```slint
// Delay for back-face expand — equals flip-duration so total flip = 2 × flip-duration.
out property <duration> flip-delay: 150ms;
```

## flashcard.slint — Restructure for Animation

Replace the current `VerticalLayout` content-swap approach with two stacked face `Rectangle`s. The outer card `Rectangle` must have `clip: true` so collapsing faces don't overflow.

### Current structure (to replace):

```slint
Rectangle { // card body
    VerticalLayout {
        if !show-back: Text { ... }  // front text
        if show-back: Text { ... }   // back text (jap small)
        if show-back: Text { ... }   // back text (explanation)
        Text { ... }                 // hint
    }
    CommonBtn { ... }  // known toggle
}
```

### New structure:

```slint
Rectangle { // card body — add clip: true
    clip: true;
    // ... background, border, border-radius unchanged ...

    TouchArea {
        clicked => {
            show-back = !show-back;
            flipped();
        }
    }

    // Front face — collapses edge-on when show-back becomes true.
    front-face := Rectangle {
        clip: true;
        width: show-back ? 0px : parent.width;
        height: parent.height;
        animate width { duration: Animations.flip-duration; easing: ease-in; }

        VerticalLayout {
            alignment: center;
            padding: 20px;
            spacing: 6px;

            Text {
                text: data.jap-obj;
                font-size: Tokens.font-size-large;
                color: Tokens.text-primary;
                horizontal-alignment: center;
            }

            Text {
                text: "Tap to reveal";
                font-size: Tokens.font-size-small;
                color: Tokens.text-secondary;
                horizontal-alignment: center;
            }
        }
    }

    // Back face — expands from zero after front collapses.
    back-face := Rectangle {
        clip: true;
        width: show-back ? parent.width : 0px;
        height: parent.height;
        animate width { duration: Animations.flip-duration; easing: ease-out; delay: Animations.flip-delay; }

        VerticalLayout {
            alignment: center;
            padding: 20px;
            spacing: 6px;

            Text {
                text: data.jap-obj;
                font-size: Tokens.font-size-body;
                color: Tokens.text-secondary;
                horizontal-alignment: center;
            }

            Text {
                text: data.explanation;
                font-size: Tokens.font-size-explanation;
                color: Tokens.text-primary;
                horizontal-alignment: center;
                wrap: word-wrap;
            }

            Text {
                text: "Tap to hide";
                font-size: Tokens.font-size-small;
                color: Tokens.text-secondary;
                horizontal-alignment: center;
            }
        }
    }

    // Known toggle — absolutely positioned, above flip faces.
    CommonBtn {
        checkable: true;
        checked <=> known;
        x: parent.width - self.width - 8px;
        y: 8px;
        width: 32px;
        height: 32px;
        text: known ? "✓" : "✗";
        accessibilityLabel: known ? "Mark as unknown" : "Mark as known";
    }
}
```

## Key Design Decisions

- **clip: true** on the outer card Rectangle prevents animating face widths from bleeding outside the card.
- **clip: true** on each face Rectangle prevents the inner `VerticalLayout` text from overflowing during the width collapse/expand.
- The known `CommonBtn` is positioned absolutely (`x:`, `y:`) above both face Rectangles — its z-order is highest and it is not clipped by the animation.
- `Animations.flip-delay` equals `Animations.flip-duration` so the back face starts expanding exactly when the front face finishes collapsing. Total flip duration = 2 × 150ms = 300ms.
- The `show-back` external property interface is unchanged — `StudySessionView` sets `session-card.show-back = false` on navigation without modification.

## Import Addition

Add `Animations` to the existing import:
```slint
import { Tokens } from "@styles";
import { Animations } from "@styles";
```

Or on one line:
```slint
import { Tokens, Animations } from "@styles";
```

## Verification

After the change, run the app (`cargo run --bin japanese_learn`) and open a flashcard study session. Tapping a card must show a smooth collapse/expand animation rather than an instant content swap. The known toggle button must remain clickable during and after the animation.
