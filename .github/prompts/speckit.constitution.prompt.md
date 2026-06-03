---
agent: speckit.constitution
---

/speckit.constitution
# General Programming Practices
- Code **must** be modular and reusable, with clear separation of concerns.
- Each task **must** have a defined scope and objectives before implementation.
- Commits **must** be atomic and descriptive; unrelated changes **must not** be included in the same commit.
- Each commit **must** pass `cargo fmt`, `cargo clippy`, builds, and tests before being pushed.
- Variable and function names **must** be descriptive; consistent naming conventions **must** be followed across files.
- Readability **must** be prioritized over cleverness; maintainability **must** be ensured.
- Automated tests **must** be implemented for core logic and UI interactions.
- Cross‑platform compatibility **must** be ensured; OS‑specific assumptions **must not** be made.
- Dependencies **must** be kept minimal to reduce binary size and deployment complexity.
- Prefer reusing existing functions and common components; do not add unnecessary functions or properties when an existing component API can be reused.
- Version control (e.g., Git) **must** be used with meaningful commit messages.

# General Workflow
- Investigate @.github/prompts/speckit.tasks.prompt.md for the list of tasks. Tasks must be completed in sequential order.
- Select the appropriate skill or agent to perform the requested task and report to the user which skill or agent is being used.
- Read @.claude/skill/implement-tasks/SKILL.md to follow the guidelines when performing a task.
- Follow language-specific coding standards defined in @.claude/rules/slint-code-style.md (Slint/UI) and @.claude/rules/rust-code-style.md (Rust) when writing or reviewing code.