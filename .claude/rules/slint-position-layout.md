# Slint Position & Layout Reference

How Slint computes size and position for plain (non-`Layout`) elements —
captured while debugging Bug 6.5 (`StudySessionView` geometry), and verified
against the generated Rust in `target/debug/build/japanese_learn-*/out/main_window.rs`.
Use this as the reference for building responsive components with dynamic
sizing instead of magic-number fixed dimensions.

## The general pattern: leaf defines size, wrapper inherits it

```slint
component ComponentB {
    // preferred-width / preferred-height here are a hint to ComponentB's
    // *own* parent — they do not constrain RectangleA below.
    RectangleA {
        // Don't set fixed width/height here either — let RectangleA's size
        // come from ITS content (Text, an inner Rectangle with a real size,
        // a component with min-width/preferred sizing, etc).
    }
}
```

**Size should originate at the leaf that has real content** (a `Text`, an
`Image`, or a component whose root declares `min-width`/explicit child sizes —
e.g. `Flashcard`'s `min-width: 200px` plus its inner `Rectangle { height: 180px }`,
or `CommonBtn`'s `preferred-width: 100px` merged with its `Text`). Every
wrapper above that leaf should avoid declaring its own size and simply let the
leaf's `layout_info` propagate upward. Confirmed in the generated code two ways:

1. **Wrappers with no declared size generate no size-tracking properties at
   all.** `CommonBtn` instances (`rectangle_211`/`220`/`226` for
   Previous/Close/Next) and the blue-bordered `Rectangle` wrapping `Flashcard`
   (`rectangle_192`) produce **zero** `layoutinfo_h`/`layoutinfo_v`/
   `layout_cache`/`width`/`height` properties — their sizes are folded into
   compile-time constants because they derive purely from static, content-driven
   constraints (`preferred-width` + `Text` font metrics; `min-width` + inner
   `Rectangle` height). No magic number, no runtime cost.
2. **The enclosing `VerticalLayout`/`HorizontalLayout` reads the leaf's
   `layout_info` directly, skipping the decorative wrapper.** The
   `VerticalLayout` row that allocates space for the Flashcard queries
   `empty_193.layoutinfo_v` (the `HorizontalLayout` wrapping `Flashcard`) —
   *not* `rectangle_192` (the wrapper `Rectangle` around it). A wrapper that
   declares no size of its own is transparent to the layout solver; it
   contributes only visual styling (border, background), and its final
   geometry is derived afterward from the cell the solver computed for its
   content.

## Size: `width`/`height` vs `preferred-*`

`width`/`height` are authoritative — the compiler folds them into
`min == max == preferred`. `preferred-*` is only a *hint*: `min` stays at the
element's natural minimum and `max` stays unbounded unless something else
constrains it.

- No children, no explicit size → `default_geometry` compiler pass makes the
  element fill its parent (`width: 100%`, `height: 100%`).
- Has content/children → preferred size is derived from them (the largest
  child preferred size, merged with any of the element's own constraints via
  the `LayoutInfo` `+` combinator).

## Position: `x`, `y` and relative position

- With no explicit `x`/`y`, an element defaults to **centered** within its
  parent (or `0,0` inside a `Layout` cell — see below).
- All positions are **relative to the immediate parent's coordinate system**.
  Absolute screen position = recursive sum of every ancestor's relative
  position, up to the top-level window. Confirmed directly in the generated
  `item_geometry`: a plain child's `y` inside a layout cell is a hard-coded
  `0.0`; the actual screen offset comes purely from composing each ancestor's
  `layout_cache` cell-origin on the way up — never from a binding on the child
  itself.
- A wrapper's own position is determined the same way, one level up, by *its*
  parent's layout.

## Why `height: 200px` "worked" but is the wrong fix

This is the mechanism behind the original `StudySessionView` overflow bug:

- **`height: 200px`** → folds into `min == max == preferred == 200px`. The
  `BoxLayout` solver has zero freedom: the cell is locked to exactly 200px.
  Predictable, but brittle — a magic number that silently mismatches the
  moment the wrapped content's real size changes.
- **`preferred-height: 200px`** → fills only `preferred`; `min` stays ≈0 and
  `max` stays unbounded. When the solver distributes leftover space among
  sibling rows, an unbounded row absorbs far more than its "preferred" hint —
  ballooning the cell. Any inner `Layout` then fills that oversized cell
  (layouts fill their parent regardless of their own preferred size),
  stretching its content and pushing siblings out of bounds.

**Key insight:** a parent does not clip or constrain a child's size by
default — an oversized or undersized child can silently overflow the parent's
visual bounds rather than being resized to fit. Sizing bugs surface as
overflow/clipping, not as layout errors.

**Takeaway:** don't wrap content in a bare `Rectangle` and guess at an
explicit or preferred size. Let the wrapper derive its size from a child that
already has a real, content-derived `layout_info` — the compiler will even
optimize the wrapper's geometry into compile-time constants when it can.
