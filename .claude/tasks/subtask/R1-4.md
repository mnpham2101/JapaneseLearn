# Subtask R1.4 — Create CommonGrid in lib/common_component

**Agent**: slint-developer  
**Parent task**: R1.4  
**Depends on**: R1.3  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this subtask is committed, `CommonGrid` exists in `lib/common_component/components/common_grid.slint` and is exported from `common_component_lib.slint`. `CommonGrid` is a responsive grid that lays out `GridItem` entries in rows of 4 columns using a `GridLayout`. Build passes with zero errors.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/common_component/components/common_grid.slint` | create | `GridItem` struct + `CommonGrid` component |
| `lib/common_component/common_component_lib.slint` | modify | add `export { CommonGrid, GridItem } from "components/common_grid.slint"` |

## Components / Modules

### `GridItem` struct

```slint
export struct GridItem {
    text: string,
    enabled: bool,
    active: bool,  // true = use primary styling (colored button)
}
```

### `CommonGrid` component

A grid that renders up to 16 `GridItem` entries arranged in 4 columns. Inactive/empty items use `enabled: false` and are hidden from layout. Active items use `primary: true` styling.

```slint
import { Tokens } from "@styles";
import { CommonBtn } from "common_button.slint";

export component CommonGrid inherits Rectangle {
    // Up to 16 configurable grid items.
    // Items with enabled=false are hidden (CommonBtn.visible = enabled).
    in property <[GridItem]> items: [];

    // Fired when a grid cell is clicked; passes the item index.
    callback item-clicked(index: int);

    GridLayout {
        spacing: Tokens.spacing-sm;
        padding: Tokens.padding-sm;

        for item[i] in items: CommonBtn {
            col: mod(i, 4);
            row: i / 4;
            text: item.text;
            enabled: item.enabled;
            primary: item.active;
            clicked => {
                item-clicked(i);
            }
        }
    }
}
```

## Patterns and Notes

- `GridLayout` in Slint uses `row` and `col` properties on children. `col: mod(i, 4)` places items left-to-right across 4 columns; `row: i / 4` advances the row every 4 items.
- Items with `enabled: false` are hidden via `CommonBtn.visible: enabled`. This means empty/placeholder slots take no layout space.
- `CommonGrid` is deliberately generic — callers (e.g. `ExerciseGrid`, `TestView`) configure the `items` array and handle `item-clicked` to open the appropriate view.
- Run `cargo build --bin japanese_learn` to confirm zero errors before committing.
