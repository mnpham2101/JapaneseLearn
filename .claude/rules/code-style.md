---
paths: 
  - .ui/**/*.slint
  - .src/**/*.rs
---

# Code Style Guidelines
- use my referred additional guides @ .github/prompts/speckit.specify.prompt.md when implementing the Slint UI components and Rust code.

# Slint declarative rules:
- Prefer two-way property binding (`<=>`) over `clicked` callbacks for state synchronization. When a child component's property directly mirrors parent state (e.g., button `checked` ↔ `known`), bind with `<=>` and set `checkable: true` to let Slint handle the toggle — no callback needed.
- Reserve callbacks for event notifications (things that "happened", like `flipped`) rather than state synchronization (value changes already captured by a property).
- This keeps UI logic declarative and eliminates redundant update paths that can diverge.