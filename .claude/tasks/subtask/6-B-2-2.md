# Subtask 6.B.2.2 — Wire generation notification + tab-switch in on_generate_exercises_clicked

**Agent**: rust-developer
**Parent task**: 6.B.2
**Depends on**: 6.B.2.1
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, clicking "Generate Flashcards" sets `VocabularyAppLogic.generation_notification` to a message naming only the newly created stack(s) (stacks whose name was not already present in the flashcard list before generation), e.g. `Flashcard stack 'N5 Verbs' generated` for one new stack, or a joined list for several — and sets `VocabularyAppLogic.active_view = 2` so the page switches to the Flashcard tab. If no new stacks were created (all names already existed), no notification is shown and the view does not switch. `cargo build` and `cargo clippy --bin japanese_learn` are clean.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/src/lib.rs` | modify | inside the existing `on_generate_exercises_clicked` handler (in `init_exercise_generator`), capture existing stack names before overwriting `flashcard_list`, diff against the newly generated `slint_stacks` names, build the notification message for the newly-created subset, and call `set_generation_notification` + `set_active_view(2)` |

## Functions / Callbacks

- Extend the existing closure registered via `logic.on_generate_exercises_clicked(move || { ... })` in `lib/vocabulary/src/lib.rs` (around line 479): before calling `flashcard_logic.set_flashcard_list(...)`, read the current stack names via `flashcard_logic.get_flashcard_list().iter().map(|s| s.stackname.to_string())` into a `HashSet<String>` (or `Vec`), then after building `slint_stacks`, filter to those whose `stackname` is **not** in that pre-existing set, and build the message string from that filtered subset only.

## Patterns and Notes

- "Newly created" means: the stack name did not exist in `flashcard_logic.get_flashcard_list()` immediately before this generation call overwrites it — not stacks that already existed under the same name.
- Message format: singular `Flashcard stack '[name]' generated.` for one new stack; for multiple, join names, e.g. `Flashcard stacks 'A', 'B' generated.` — keep it simple and consistent, exact wording is the implementer's call as long as it names only the new stacks.
- If the filtered "new stacks" set is empty, do not call `set_generation_notification` with an empty/blank message and do not switch tabs — leave `active_view` and `generation_notification` untouched so existing behavior for "regenerate with no new lessons" stays a no-op notification-wise.
- Use `vocab_logic.set_generation_notification(...)` and `vocab_logic.set_active_view(2)` — both globals (`VocabularyAppLogic`, `FlashcardAppLogic`) are already obtained in this handler.
- This is the call-site commit completing the chain (6.B.2.1 declared the properties; this wires them) — run `cargo clippy --bin japanese_learn` and confirm zero warnings before committing.
