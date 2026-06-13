# Subtask 6.S.1 — Apply day-mode color palette and add bar-level token distinction

**Agent**: slint-developer  
**Parent task**: 6.S.1  
**Depends on**: none (first task in Phase 6.S)  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `lib/styles/tokens.slint` holds the new day-mode palette and exposes two new bar-level tokens (`topic-tab-bar-bg`, `action-bar-bg`, `action-tab-btn-active-bg`) so that the StudyPage topic tab row and the VocabularyPage action bar can be independently styled. `study_page.slint` uses `topic-tab-bar-bg` and `vocabulary_page.slint` uses `action-bar-bg`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed

| File | Action | What changes |
|---|---|---|
| `lib/styles/tokens.slint` | modify | Replace/update color values; add 3 new tokens |
| `ui/pages/study_page.slint` | modify | Use `Tokens.topic-tab-bar-bg` for topic tab row |
| `lib/vocabulary/ui/pages/vocabulary_page.slint` | modify | Use `Tokens.action-bar-bg` for action bar |

## Color Palette Mapping

Apply the six customer-specified colors to semantic tokens:

| Token | New value | Rationale |
|---|---|---|
| `page-background` | `#FEFAE0` | Cream — main page canvas |
| `card-container-background` | `#FAEDCD` | Warm peach — card/form surfaces |
| `input-form-background` | `#FAEDCD` | Same peach — inline create forms |
| `card-field-background` | `#FEFAE0` | Cream — input text fields |
| `nav-background` | `#251504` | Dark brown — active topic tab, nav bar |
| `text-primary` | `#251504` | Dark brown — all primary text |
| `text-secondary` | `#D4A373` | Warm tan — secondary/caption text |
| `border-color` | `#D4A373` | Warm tan — card and input borders |
| `nav-bar-background` | `#D4A373` | Warm tan — StudyPage topic tab row |
| `stack-label-bg` | `#FAEDCD` | Peach — lesson/stack label rows |
| `stack-label-border` | `#D4A373` | Tan — label borders |
| `stack-label-text` | `#251504` | Dark brown — label text |
| `input-background` | `#FEFAE0` | Cream — text inputs |
| `input-border` | `#D4A373` | Tan — input borders |
| `drag-target-highlight` | `#CCD5AE` | Sage — drag-over highlight |
| `btn-default-bg` | `#E9EDC9` | Light sage — default button resting |
| `btn-hover-bg` | `#CCD5AE` | Sage — hover state |
| `btn-pressed-bg` | `#D4A373` | Tan — pressed/held state |
| `btn-default-text` | `#251504` | Dark brown — button label |
| `btn-primary-bg` | `#D4A373` | Tan — primary (confirm/generate) |
| `btn-primary-hover-bg` | `#C08040` | Darker tan — primary hover |
| `btn-primary-pressed-bg` | `#A06820` | Deep tan — primary press |
| `known-bg` | `#E9EDC9` | Sage — known card |
| `known-border` | `#CCD5AE` | Sage border |
| `known-icon-color` | `#251504` | Dark brown — known ✓ |
| `unknown-bg` | `#FAEDCD` | Peach — unknown card |
| `unknown-border` | `#D4A373` | Tan — unknown border |

## New tokens to add

```slint
// ── Bar-level distinction ─────────────────────────────────────────────────────
// StudyPage topic tab row (top-level navigation: Vocabulary / Grammar / Reading)
out property <color> topic-tab-bar-bg: #D4A373;
// VocabularyPage action bar (secondary navigation: Lesson / Exercise / Flashcard)
out property <action-bar-bg> action-bar-bg: #CCD5AE;
// Active button color for VocabularyPage action bar tabs (distinct from topic-tab active = nav-background)
out property <color> action-tab-btn-active-bg: #D4A373;
```

> Note: the property type for `action-bar-bg` is `<color>` — the comment above has a typo. Correct it.

## StudyPage change

In `ui/pages/study_page.slint`, the topic tab row Rectangle currently uses `Tokens.nav-bar-background`. Update:
```slint
// before
background: Tokens.nav-bar-background;

// after
background: Tokens.topic-tab-bar-bg;
```

## VocabularyPage change

In `lib/vocabulary/ui/pages/vocabulary_page.slint`, the action bar Rectangle uses `Tokens.nav-bar-background`. Update:
```slint
// before
background: Tokens.nav-bar-background;

// after
background: Tokens.action-bar-bg;
```

## Patterns and Notes

- **Do not** remove or rename existing token names — downstream files reference them. Only change values and add new tokens.
- `text-on-dark` (used as button text color in CommonBtn) should remain appropriate for the new checked background. `#FEFAE0` (cream) works as text on `#251504` (dark brown). Update `text-on-dark: #FEFAE0` if it was previously `#020410`.
- Verify `cargo build --bin japanese_learn` passes with zero errors after the change.
- Run `cargo fmt` before committing.
