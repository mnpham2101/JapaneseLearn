# Subtask 6.T1.3 — Author `theme_solarized_light.slint`

**Agent**: slint-developer
**Parent task**: 6.T1
**Depends on**: 6.T1.2
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `lib/styles/themes/theme_solarized_light.slint` exists as a complete, standalone `export global Tokens { ... }` definition — identical in property names and types to `theme_default.slint`, but with the Solarized Light color values listed below. This file is authored only; it is **not** wired into `styles.slint` (the default theme remains active, zero regression). Build is green; `cargo run` still shows the default palette.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/styles/themes/theme_solarized_light.slint` | create | new file: `export global Tokens { ... }` with every property from `theme_default.slint`, colors replaced per the mapping table below, all non-color tokens copied verbatim |

## Components / Modules

- **`Tokens` global (Solarized Light variant)** — a complete, drop-in-compatible alternative palette. Same property set as `theme_default.slint`; only `<color>` values differ (per table below). All `<length>` tokens (border-width, border-radius, card-container-border-radius, spacing-*, font-size-*) are copied unchanged from `theme_default.slint`.

## Functions / Callbacks

None — pure data/token authoring, no logic.

## Color Mapping Table (apply verbatim — already derived from the user's live VSCode Solarized Light theme)

| Property | New value | Property | New value |
|---|---|---|---|
| border-color | #D3AF86 | known-bg | #EEE8D5 |
| card-field-background | #FDF6E3 | known-border | #859900 |
| card-container-background | #EEE8D5 | known-icon-color | #586E75 |
| card-container-border-color | #D3AF86 | unknown-bg | #FDF6E3 |
| text-primary | #586E75 | unknown-border | #CB4B16 |
| text-secondary | #657B83 | unknown-icon-color | #93A1A1 |
| text-placeholder | #93A1A1 | page-background | #FDF6E3 |
| text-on-dark | #FDF6E3 | nav-background | #073642 |
| btn-default-bg | #EEE8D5 | nav-bar-background | #586E75 |
| btn-default-text | #586E75 | input-background | #FDF6E3 |
| btn-default-border | #D3AF86 | input-border | #D3AF86 |
| btn-hover-bg | #DFCA88 | input-text | #586E75 |
| btn-hover-text | #586E75 | input-form-background | #DDD6C1 |
| btn-pressed-bg | #B58900 | stack-label-bg | #EEE8D5 |
| btn-pressed-text | #FDF6E3 | stack-label-border | #D3AF86 |
| btn-disabled-bg | #EEE8D5 | stack-label-text | #586E75 |
| btn-disabled-text | #93A1A1 | drag-target-highlight | #DFCA88 |
| btn-primary-bg | #268BD2 | topic-tab-bar-bg | #586E75 |
| btn-primary-text | #FDF6E3 | action-bar-bg | #DDD6C1 |
| btn-primary-hover-bg | #1E6FA8 | action-tab-btn-active-bg | #B58900 |
| btn-primary-pressed-bg | #15527D | | |

## Patterns and Notes

- Read `theme_default.slint` first to get the exact property declaration order, types, comments structure, and the full list of non-color tokens to copy verbatim.
- All properties not listed in the mapping table (border-width, border-radius, card-container-border-radius, spacing-md/sm/xs, padding-lg/sm, font-size-*) are **not colors** — copy them unchanged from `theme_default.slint`.
- Do **not** modify `styles.slint` in this commit — the new theme file must exist but stay unwired, so the running app continues to show the default palette (zero regression).
- This file establishes the second swappable theme; future theme switches will follow the same one-line `export { Tokens } from "themes/theme_solarized_light.slint";` pattern demonstrated structurally by 6.T1.2.
