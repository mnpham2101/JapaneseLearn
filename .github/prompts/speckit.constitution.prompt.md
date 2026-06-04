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
- Slint component names **must** use PascalCase with a functional suffix: `*Btn` (button), `*TxtBox` (text input), `*Container` (layout/Rectangle), `*Page` (page view).
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

# General Architecture
- Follow MVC architecture, but keep the common folder structure as other Slint-Rust applications: 
  - Slint components must be in together, but keep the folder name as 'ui/components/'. The entry Slint component must be in folder 'ui'
  - All public model data struct should be defined in 'ui/model/'
  - All backend business logic should be in 'src/'
- If the Slint components and its Rust backend logic are built as library, keep them in '/lib'. 
- Folder structure: 
```
src/
  main.rs                       # entry point only — no business logic here
ui/
  main_window.slint             # root window, page routing
  components/                   # main application components - one component per file
  ui/styles                     # reusable style definitions
  model/                        # main application data models
  pages/                        # page-level components (study_page, review_page)
lib/
  libA/                           # Slint component and backend logic built as library
    src/lib.rs                    # entry point to init function
    src/*.rs                      # Rust backend logic supporting UI manipulation
    ui/components                 # reusable widgets — one component per file 
    ui/model                      # reusable data models written in Slint
    ui/main_lib.slint             # entry slint library file - export the library components and models                
    build.rs                      # entry point to build library
    Cargo.toml                    # define building codes as package
  libB/                           # Rust logic built as service library
    src/lib.rs                    # Rust backend logic
    src/*.rs                      # Rust backend logic, could include helper functions, classes, etc 
    main.rs                       # entry point to build library
    Cargo.toml                    # define building codes as package
build.rs
Cargo.toml
```