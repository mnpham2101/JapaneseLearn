# Slint Flickable with @children Pattern

## The rule

When a component uses `Flickable` to scroll `@children`, the **size constraint (height/width) must live on the same element that directly contains the Flickable** — and that element must be the **component root itself** (via `inherits Rectangle`). Never wrap the Flickable in an intermediate child `Rectangle`.

## Why the intermediate wrapper breaks layout

`Flickable { viewport-height: content.preferred-height }` computes a virtual size from its content (e.g. 2000 px for 20 items). This virtual size propagates upward through the layout tree as the preferred-height of the Flickable's ancestors.

- **Working (constraint on root):** The root's explicit `height` clamps the preferred-height before it reaches the parent layout. The parent sees `preferred-height = root.height` → correct position.
- **Broken (constraint on inner Rectangle):** The inner Rectangle's `height` clamps locally, but the component root has no constraint. The 2000 px virtual size escapes the wrapper and becomes the root's preferred-height. The parent `VerticalLayout` then allocates 2000 px for the component and positions it at y = 0, visually jumping the component to the top of the screen.

## Working pattern

```slint
component ScrollableList {
    height: 300px;              // constraint is ON THE ROOT

    Flickable {                 // Flickable is a DIRECT child of root
        viewport-height: list.preferred-height;
        vertical-stretch: 1;
        list := VerticalLayout {
            alignment: start;
            // ... static children ...
            @children           // injected children go here
            // ... more static children ...
        }
    }
}
```

If the component also needs a visual border/background, use `inherits Rectangle` on the root rather than adding an inner `Rectangle` wrapper:

```slint
component ScrollableList inherits Rectangle {
    height: 300px;
    border-radius: Tokens.border-radius;
    border-width: Tokens.border-width;
    border-color: Tokens.border-color;
    background: Tokens.page-background;

    Flickable {
        viewport-height: list.preferred-height;
        vertical-stretch: 1;
        list := VerticalLayout {
            padding: Tokens.padding-lg;
            spacing: Tokens.spacing-md;
            alignment: LayoutAlignment.stretch;
            @children
        }
    }
}
```

## Anti-pattern (forbidden)

```slint
// WRONG — intermediate Rectangle between root and Flickable
component ScrollableList {
    Rectangle {                 // ← wrapper NOT the root
        height: 500px;          // constraint here does NOT prevent viewport leak
        Flickable {
            viewport-height: list.preferred-height;
            list := VerticalLayout {
                @children
            }
        }
    }
}
```

## Checklist before committing a scrollable component

1. Does `Flickable` appear as a **direct child of the component root**? ✓
2. Is the **height/width constraint on that same root**? ✓
3. Does the root use `inherits Rectangle` instead of a child `Rectangle` for visual styling? ✓
4. Is there **no intermediate wrapper** (`Rectangle`, `VerticalLayout`, etc.) between root and `Flickable`? ✓

## Additional: removing redundant wrappers at call sites

Even a correctly built scrollable component can break if the **call site** wraps it in an unsized intermediate element. For example:

```slint
// WRONG at call site — Rectangle with max-width wrapping a Flickable-based component
Rectangle {
    max-width: 330px;
    ScrollableList { ... }   // viewport leak propagates through this Rectangle too
}
```

Remove the intermediate wrapper and let the scrollable component receive its size directly from the parent layout:

```slint
// CORRECT at call site — no wrapper
HorizontalLayout {
    alignment: center;
    ScrollableList {
        width: min(parent.width - 32px, 480px);
    }
}
```
