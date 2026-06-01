<!--
Sync Impact Report
Version change: none → 1.0.0
Modified principles: UI and Slint Separation, Rust Safety and Modular Design, Incremental Quality and Git Discipline, Testability and Cross-Platform Reliability, Minimal Dependencies and Performance
Added sections: Technology Constraints, Development Workflow
Removed sections: none
Templates reviewed: ✅ .specify/templates/plan-template.md, ✅ .specify/templates/spec-template.md, ✅ .specify/templates/tasks-template.md, ✅ .specify/templates/constitution-template.md
Follow-up TODOs: none
-->

# japanese-learn Constitution

## Core Principles

### UI and Slint Separation
UI definitions MUST be separated into `.slint` files and Rust logic MUST not be mixed with UI markup. Property bindings MUST be used instead of imperative updates whenever possible, and the `changed` keyword MUST be used to react to property changes.
Rationale: separating UI declarations from Rust logic keeps the interface maintainable, testable, and aligned with Slint best practices.

### Rust Safety and Modular Design
Immutability MUST be favored; `let` MUST be used over `let mut` unless mutation is essential. Error handling MUST use `Result` and `Option`; panics MUST not be used for normal control flow. Code MUST be organized into modules and crates with small focused functions, and UI logic, business logic, and data access MUST be separated.
Rationale: clean modular design prevents coupling between UI, domain logic, and data access, making the application easier to reason about and evolve.

### Incremental Quality and Git Discipline
Commits MUST be atomic, descriptive, and scoped to a single objective. Every commit MUST pass builds and tests before being pushed, and unrelated changes MUST not be included in the same commit. `cargo fmt` and `clippy` MUST be run regularly to enforce style and catch common issues.
Rationale: disciplined version control and automated quality checks reduce regressions and make review faster.

### Testability and Cross-Platform Reliability
Automated tests MUST cover core logic and UI interactions. Cross-platform compatibility MUST be ensured, and OS-specific assumptions MUST not be made. The application MUST be validated on all intended target platforms, including desktop and WebAssembly when applicable.
Rationale: verifying behavior across target platforms upholds the project's goal of reliable, maintainable learning software.

### Minimal Dependencies and Performance
Third-party libraries MUST be integrated through `Cargo.toml` carefully, and only well-maintained crates SHOULD be used. Dependencies MUST be kept minimal to reduce binary size and deployment complexity. Rendering MUST be optimized for lightweight performance, and unnecessary animations or large assets MUST not be used.
Rationale: keeping dependencies small and performance-conscious maintains a fast, portable experience suited to the project.

## Technology Constraints
The project MUST use Rust and Slint for UI. UI components such as custom buttons, list items, and reusable widgets MUST be defined in separate `.slint` files and imported where needed. Dynamic data sources MUST be integrated only after the program builds successfully with hardcoded default models.
Rationale: explicit technology constraints ensure the project stays aligned with its chosen stack and avoids premature coupling of dynamic data.

## Development Workflow
Development work MUST follow a clear sequence: define scope, implement incrementally, and validate each change with tests and formatting checks. Code reviews MUST verify adherence to the constitution, including separation of UI markup from Rust logic, modular design, and commit discipline. Feature branches MUST be used for new work, and pull requests MUST be reviewed before merge.
Rationale: a disciplined workflow supports high-quality output and reduces the risk of merged technical debt.

## Governance
This constitution supersedes informal practices and is the primary source of project development rules. Amendments require documentation, review, and a follow-up update to relevant templates and checklists. Compliance review MUST occur for all feature branches and pull requests that change UI structure, project architecture, or core workflow rules.
Versioning follows semantic versioning for the constitution itself: initial adoption at 1.0.0, with major bumps for incompatible governance changes, minor bumps for new principles or sections, and patch bumps for wording clarifications.

**Version**: 1.0.0 | **Ratified**: 2026-06-01 | **Last Amended**: 2026-06-01

