# Bug description

All though there are default Vocabulary data, but there is no lesson list in the page `Vocabulary`. 
A console log has been added to init function; console log at startup indicate that the lesson-list size it zero:
```
Current lesson size:  0
```
The should load avaible lessons from `@vocabulary/ui/data/*.json` file. If there are multiple files, each should be a lessons, displayed in LessonStackList.

After resolving, a planuml file should be generated to document the call flow, and how lesson data are loaded.

# Analysis

## Confirmed symptom

`vocabulary::init` correctly loads ~18 lessons (from persisted `vocabulary.json`,
or seeded from `lib/vocabulary/ui/data/*.json` via `load_and_save_defaults()` on
first run) and calls `logic.set_lesson_list(lessons_to_slint(&lessons))`.
`logic.get_lesson_list().row_count()` returns `18` immediately afterward — the
Rust side is correct. Yet `LessonStackList`'s `for lesson[i] in
VocabularyAppLogic.lesson-list` renders nothing, and a `Timer`/`changed`
debug instrument inside the `.slint` component shows
`VocabularyAppLogic.lesson-list.length == 0` continuously, before, during,
and after `set_lesson_list` runs. A live screenshot confirmed an empty
lesson list in the Vocabulary tab.

Three plausible causes were tested and disproven before reaching the real one:
1. *Timing* — debug print fires before `vocabulary::init` runs. Disproven: a
   `Timer` polling every 1.5 s kept reporting `length == 0` for the entire
   session, long after `init()` completed.
2. *Stale incremental build* — `cargo clean -p japanese_learn -p vocabulary`
   plus a full rebuild reproduced the identical symptom.
3. *Duplicate global instance* — `grep -rln "global VocabularyAppLogic"`
   confirmed exactly one declaration exists, and `lib/flashcard` uses an
   identical dual-path import pattern (`"../X.slint"` vs `"X.slint"`)
   successfully, ruling out an import-resolution split.

## Root cause — confirmed by inspecting the Slint-generated Rust code

`lib/vocabulary/ui/vocabulary_app_logic.slint` declared the property with **no
binding/initializer**:
```slint
in-out property <[VocabularyLessonModel]> lesson-list;
```
while the working analogue, `FlashcardAppLogic.flashcard-list` (in
`lib/flashcard/ui/flashcard_app_logic.slint`), declares an **explicit** binding
(a hardcoded sample-data array).

To prove this was the cause (not just correlation), the property declaration
was toggled between the no-binding form and an explicit `: []` form, and the
generated Rust was diffed each time
(`target/debug/build/{vocabulary,japanese_learn}-*/out/{vocabulary_lib,main_window}.rs`):

**Without a binding** (`lesson-list;`) — `InnerVocabularyAppLogic::init()`
contains **no statement at all** for `lesson_list` (every other property gets
either `.set(default)` or `set_property_binding(...)`; `lesson_list` gets
neither). And in `main_window.rs`, the repeater backing `LessonStackList`'s
`for` loop is generated as:
```rust
_self.repeater0.set_model_binding({
    let self_weak = ...;
    move || {
        ...
        (sp::ModelRc::new(sp::VecModel::<VocabularyLessonModel>::from(sp::vec![]))) as _
    }
});
```
— a closure that **never reads `InnerVocabularyAppLogic::lesson_list`**; it
just constructs a fresh empty `ModelRc` every time. Every reference to
`VocabularyAppLogic.lesson-list` anywhere in `.slint` code was eliminated
(`grep -c lesson_list main_window.rs` → `0` matches against the global).

**With an explicit binding** (`lesson-list: [];`) — `init()` installs a real
binding via `set_property_binding(...)`, and the repeater closure becomes a
genuine reactive property read:
```rust
_self.repeater0.set_model_binding({
    let self_weak = ...;
    move || {
        ...
        ({ *&InnerVocabularyAppLogic::FIELD_OFFSETS.r#lesson_list() }
            .apply_pin(_self.globals.get().unwrap().global_VocabularyAppLogic.as_ref())
            .get()) as _
    }
});
```
This is structurally identical to how `FlashcardAppLogic.flashcard-list`
(which *does* declare an explicit default) is compiled — confirmed by reading
`target/debug/build/flashcard-*/out/flashcard_lib.rs`, whose `init()` also
calls `set_property_binding` for `flashcard_list`.

**Why the compiler does this:** when an `in-out` property has no declared
binding, the Slint compiler can statically prove — by analyzing only the
`.slint` source (it cannot see `set_lesson_list()` calls made from Rust,
since those live outside its analysis scope) — that the property's value is
never assigned anything other than its type's default (`[]`). It therefore
treats every read of that property as the compile-time constant `[]`,
constant-folds it at each use site, and skips emitting any runtime
`Property` storage, binding, or dependency-tracking code for it — the same
const-folding optimization documented in `.claude/rules/slint-position-layout.md`
for size/position properties with no explicit binding ("wrappers with no
declared size generate no size-tracking properties at all... folded into
compile-time constants"). Declaring an explicit binding — even a trivial
`: []` — defeats this optimization: it tells the compiler the property *can*
hold a runtime-computed value, so it allocates a real `Property` cell, wires
it into the reactive dependency graph, and lets `set_lesson_list()` (which
calls `.set(value)` on that cell from Rust) actually take effect and notify
the `for` loop's repeater.

# Solution

Add an explicit (trivial) binding to `lesson-list` so the Slint compiler
allocates a genuine reactive `Property` slot instead of constant-folding the
property to `[]`:

```diff
- in-out property <[VocabularyLessonModel]> lesson-list;
+ in-out property <[VocabularyLessonModel]> lesson-list: [];
```

This mirrors the established working pattern (`flashcard-list` has an
explicit binding), is a one-line surgical fix, and requires no changes to
`init()`, `lessons_to_slint()`, or any loading logic — all of which were
already correct. Verified: build green, `vocabulary-tests` (14) and
`flashcard-tests` (8) pass, `cargo fmt`/`cargo clippy --bin japanese_learn`
clean (zero new warnings), and a live screenshot shows the lesson rows
("N5 Verbs — Eating...", "N5 Verbs — M...", ...) rendering correctly.

Call flow documented in `speckit.bug.6.4.callflow.puml` per the report's
requirement.