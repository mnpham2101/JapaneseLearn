# Subtask 6.D.3.2 — Wire restore-defaults-clicked handler

**Agent**: rust-developer  
**Parent task**: 6.D.3  
**Depends on**: 6.D.3.1  
**Commit**: one commit, one logical change

---

# Subtask Goals

After this commit, clicking "Restore Defaults" in the UI clears all lessons from `VocabularyAppLogic`, reloads the three embedded JSON defaults via `include_str!()`, pushes the combined list back to the logic, and saves to `vocabulary.json`. `cargo clippy` must report zero warnings.

---

# Subtask Technical Approach

## Files to be Changed or Added

| File | Action | What changes |
|---|---|---|
| `lib/vocabulary/src/lib.rs` | modify | register `on_restore_defaults_clicked` handler in `init()` |

## Functions / Callbacks

- `on_restore_defaults_clicked` closure in `init()`:
  ```rust
  {
      let ui_weak = ui.as_weak();
      logic.on_restore_defaults_clicked(move || {
          #[cfg(not(target_arch = "wasm32"))]
          {
              let defaults = load_and_save_defaults();
              let ui = ui_weak.unwrap();
              let logic = ui.global::<VocabularyAppLogic>();
              logic.set_lesson_list(lessons_to_slint(&defaults));
          }
      });
  }
  ```
- `load_and_save_defaults()` is already committed in 6.D.2 — this task only adds the `on_restore_defaults_clicked` call-site registration.

## Patterns and Notes

- This is the chain-completing commit for the 6.D.3 feature. `cargo clippy` must pass with zero warnings.
- The handler is gated `#[cfg(not(target_arch = "wasm32"))]` because `load_and_save_defaults()` and `save_vocabulary()` are both desktop-only.
- On WASM the callback is registered but its body is empty (the `#[cfg]` block is absent), so no functionality is lost.
- `load_and_save_defaults()` both saves `vocabulary.json` and returns the combined `Vec<LessonData>` — no separate save call is needed inside the handler.
