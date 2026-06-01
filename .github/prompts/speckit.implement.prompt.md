---
agent: speckit.implementation
---
/speckit.implementation

# Implementation Guidelines
- Implementations are derived from tasks defined in `speckit.tasks`, which themselves are based on `speckit.plan` and `speckit.specify`.
- Each implementation **must** follow the best practices outlined in `speckit.constitution` (UI separation, modularity, immutability, error handling).
- Each implementation **must** be atomic: focused on a single task objective, producing a clear deliverable.
- Code **must** compile successfully on Windows and WebAssembly before commit. Run build commands for both targets, prompt verification of successful compilation, and manually test UI interactions on both platforms.
- Rust unit tests and integration tests **must** pass before commit; UI interactions **must** be manually tested on both targets.
- After successful implementation, a commit **must** be created with a descriptive message, and code review **must** be performed before moving on.
- Implementations **must** reference the task ID or phase they belong to, ensuring traceability.
- Documentation **should** be updated alongside code changes to reflect architecture and usage.
- Future backlog items **may** be implemented after core phases are complete, without disrupting existing modules.

# Implementation Workflow
1. Select a task from `speckit.tasks`.
2. Implement the task in Rust + Slint according to constitution rules.
3. Clearly reference the task ID in the commit message (e.g., "Implement add flashcard functionality [Task 2.2]" where the first digit is the phase number, and the second digit is the task number).
4. Run build for Windows and WebAssembly targets.
5. Run Rust unit/integration tests; perform manual UI validation.
6. Commit changes with descriptive message referencing task ID.
7. Submit for review and approval before proceeding to the next task.