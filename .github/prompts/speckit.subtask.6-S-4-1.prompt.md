# Subtask 6.S.4.1 — Kanji flashcard: is-kanji field in FlashcardModel + large front rendering

**Agent**: slint-developer  
**Parent task**: 6.S.4  
**Depends on**: 6.S.3  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, `FlashcardModel` has an `is-kanji: bool` field and `flashcard.slint` renders the front face with an extra-large, calligraphic font when `is-kanji` is true. A new `font-size-kanji` token is added to tokens.slint. Build passes; the field defaults to `false` so all existing cards are unaffected until the Rust side is updated in 6.S.4.2.

---

# Subtask Technical Approach

## Files to be Changed

| File | Action | What changes |
|---|---|---|
| `lib/flashcard/ui/model/flashcard_model.slint` | modify | Add `is-kanji: bool` field to `FlashcardModel` |
| `lib/styles/tokens.slint` | modify | Add `font-size-kanji` token |
| `lib/flashcard/ui/components/flashcard.slint` | modify | Conditional font-size and font-family on front face Text |

## FlashcardModel Change

```slint
// lib/flashcard/ui/model/flashcard_model.slint
export struct FlashcardModel {
    jap-obj: string,
    explanation: string,
    known: bool,
    is-kanji: bool,   // ← add this field
}
```

Default value for `is-kanji` is `false` in all Slint struct literals — Slint zero-initializes bool fields, so existing code that constructs `FlashcardModel { jap-obj: ..., explanation: ..., known: ... }` will compile with `is-kanji` defaulting to `false`.

## tokens.slint Change

Add a kanji-specific font size token:

```slint
// Extra-large font for kanji front face — brush-stroke calligraphic rendering.
out property <length> font-size-kanji: 72px;
```

## flashcard.slint — Front Face Text Update

Inside the `front-face` Rectangle added in task 6.S.3, update the japanese-word `Text` element:

```slint
Text {
    text: data.jap-obj;
    font-size: data.is-kanji ? Tokens.font-size-kanji : Tokens.font-size-large;
    font-family: data.is-kanji ? "MS Mincho" : "";
    font-weight: data.is-kanji ? 400 : 400;
    color: Tokens.text-primary;
    horizontal-alignment: center;
    vertical-alignment: center;
}
```

**Font family reasoning**: `"MS Mincho"` (明朝体) is the standard Windows serif/calligraphic Japanese font available on all Windows systems. It resembles brush strokes for kanji characters. An empty `font-family: ""` falls back to the system default.

> If the build target ever expands to macOS/Linux, additional fallbacks can be chained: `data.is-kanji ? "MS Mincho,Hiragino Mincho ProN,Noto Serif CJK JP" : ""`. Keep the Windows font first.

## Flashcard default data update

The default data value for `Flashcard` in `flashcard.slint` uses:
```slint
in property<FlashcardModel> data: { jap-obj: "", explanation: "", known: true };
```

Update to include the new field (it defaults to false but be explicit for clarity):
```slint
in property<FlashcardModel> data: { jap-obj: "", explanation: "", known: true, is-kanji: false };
```

## Rust Persistence Impact

The `CardData` struct in `lib/flashcard/src/lib.rs` (used for JSON persistence) does **not** need to change in this task — that is handled by the rust-developer in task 6.S.4.2. Since `is-kanji` defaults to `false` when deserializing existing JSON (serde skip_serializing_if + default), existing saved data will load correctly.

## Verification

Run `cargo build --bin japanese_learn`. The build must pass with zero errors. All existing flashcard stacks should display normally (is-kanji=false for all). No visual change until the Rust side (6.S.4.2) sets is-kanji=true for kanji-front generated cards.
