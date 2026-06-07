---
paths: 
  - ui/**/*.slint
---
# Slint naming convention
- Component names **must** use PascalCase with a functional suffix:
    - `*Btn` for button (e.g., `CommonBtn`, `NavBtn`).
    - `*TxtBox` for text input/text box (e.g., `JapaneseTxtBox`).
    - `*Container` for general layout containers and Rectangles (e.g., `FlashcardContainer`).
    - `*Page` for main view pages (e.g., `StudyPage`, `ReviewPage`).
# Slint Best Practices
- UI definitions **must** be separated into `.slint` files; Rust logic **must not** be mixed with UI markup.
- Each UI component **must** be defined in its own `.slint` file and imported where needed to promote reusability and maintainability.
- Any logical sub-view, panel, or form with its own state or callbacks **must** be extracted as a named component in its own file — even if it is only used once. Inline anonymous blocks inside a `*Page` file are only acceptable for simple one-liner layout wiring.
  - **Anti-pattern (forbidden):** inlining a full sub-view directly inside a `*Page`:
    ```slint
    // study_page.slint — WRONG
    if show-session: VerticalLayout {   // ← this block is a component; give it a name and a file
        Text { text: progress; }
        Flashcard { ... }
        HorizontalLayout { CommonBtn { ... } ... }
    }
    ```
  - **Correct pattern:** extract to a named component in its own file, import and reference it:
    ```slint
    // ui/pages/study_session_view.slint — NEW file
    export component StudySessionView { ... }

    // study_page.slint — reference the component
    import { StudySessionView } from "study_session_view.slint";
    if show-session: StudySessionView { ... }
    ```
  - The file name **must** be the kebab-case of the component name: `StudySessionView` → `study_session_view.slint`.
- Property bindings **must** be used instead of imperative updates wherever possible.
- The `changed` keyword **must** be used to react to property changes.
- Component hierarchies **should** remain shallow; reusable custom components **must** be preferred over duplication.
- Common UI components (buttons, lists, etc.) **must** be defined with base properties and behaviors, and **should** be extended for specific use cases.
- Specific UI components (e.g., custom buttons, list items) **must** be defined in separate `.slint` files and imported where needed.
- Components requiring data models (e.g., list views, `HorizontalView`) **must** have default hardcoded models first; the program **must** build successfully before integrating dynamic data sources. Code **must** be committed after each successful UI component implementation.
- Rendering **must** be optimized for lightweight performance; unnecessary animations or large assets **must not** be used.
- UI responsiveness **must** be tested across desktop, mobile, and WebAssembly targets.
- Avoid hardcode sizes. Let the UI responses to different screensizes. 
- UI styling must be universal accross all components. Define common size, colors in a separate slint file.
## Slint declarative rules:
- Prefer two-way property binding (`<=>`) over `clicked` callbacks for state synchronization. When a child component's property directly mirrors parent state (e.g., button `checked` ↔ `known`), bind with `<=>` and set `checkable: true` to let Slint handle the toggle — no callback needed.
- Reserve callbacks for event notifications (things that "happened", like `flipped`) rather than state synchronization (value changes already captured by a property).
- This keeps UI logic declarative and eliminates redundant update paths that can diverge.

# Slint reference code for patterns mentioned above:
## Design common button:
* check slint document for behavior of `enabled`, `checked`, `checkable`, `pressed` properties and add same properties and logic to common_button. Keep the name, and comments:
```
// button could be disable. Callback can only be invoked when enabled is true.
in-out property <bool> enabled <=> ta.enabled: true;
// if the button is clicked, button state changes. Useful for toggle button, in flashcard
in-out property <bool> checked: false;
// if the button is checkable, it can be checked or unchecked when clicked. Otherwise, it behaves like a normal button without checked state. Useful for mutual exclusive button group, in flashcard stack list.
in property <bool> checkable: false;
// if the button is pressed, it is being clicked. Useful for visual feedback when user clicks and holds the button.
out property <bool> pressed: false;
ta:=TouchArea {
  clicked => {
    if (checkable) {
        checked = !checked;
    }
  }
}
```
* implement "accessible" properties  in slint.
* all specialized buttons (e.g., flashcard known/unknown toggle, stack selection) should inherit from the common button and extend it with specific properties and behaviors as needed.

## Prefer declarative property binding over imperative callbacks:
* When a child component's property directly mirrors parent state, prefer two-way binding (`<=>`) over a `clicked` callback. Example — instead of:
  ```slint
  CommonButton {
      checked: known;
      clicked => { known = !known; known-toggled(known); }
  }
  ```
  prefer:
  ```slint
  CommonButton {
      checkable: true;
      checked <=> known;
  }
  ```
  Slint's `checkable` mechanism toggles `checked` on click; the two-way binding propagates the change to `known`. No callback needed.
* Reserve callbacks for event notifications (actions without a dedicated state property, e.g., `flipped`). Do not use callbacks to synchronize properties that can be bound directly.

## Mutually exclusive button group pattern:
* For a group of buttons where only one can be active (e.g., flashcard stack selection), use `checkable: true` on each button and bind their `checked` properties to a shared state that tracks the selected stack. This allows Slint to manage the exclusivity without manual state toggling in callbacks. Follow the pattern:
```slint
export component Page {
    property <int> current-button-index: 0;

    HorizontalLayout {
      for item in stackList: CommonButton {
          text: item.name;
          checkable: true;
          checked: (current-button-index == self.index) ? true : false;  // bind the checked state to the current index of the parent component
          clicked => { current-button-index = self.index; }   // set current index on click, Slint will handle the checked state based on the binding
      }
    }
}
``` 

## Vertically stacked up components pattern:
* To stack components vertically, use a `VerticalLayout` or `VerticalBox` container. This ensures consistent spacing and alignment without manual positioning. 
* To enable showing/hiding components, set `alignment: LayoutAlignment.end` on the container so that components stack up from the bottom and are removed from top to bottom as they are hidden.
* Notice how state changes (e.g., `show-a`, `show-b`, `show-c`) control the visibility and height of the rectangles, creating a smooth stacking effect as components are shown or hidden. No Slint built-in popup is used. No imperative callback is needed to manage the layout; it's all driven by declarative property bindings and layout management.

Example:
```slint
import { AboutSlint, Button } from "std-widgets.slint";

export component Page inherits VerticalLayout {
    alignment: LayoutAlignment.end;

    property <bool> show-a <=> show-a-button.checked;
    property <bool> show-b: self.show-a && show-b-button.checked;
    property <bool> show-c: self.show-a &&  show-c-button.checked;

    if root.show-c: Rectangle {
        background: Colors.magenta;
        height: self.visible ? 20px: 0px;
        visible: root.show-c;
    }

    if root.show-b: Rectangle {
        background: Colors.yellow;
        height: self.visible ? 25px : 0px;
        // visible: root.show-b;

    }

    Rectangle {
        background: Colors.beige;
        height: 75px;
        visible: root.show-a;

        HorizontalLayout {

            height: 25px;

        show-b-button := Button {
            text: "Show B";
            checkable: true;
        }

        show-c-button := Button {
            text: "Show C";
            checkable: true;
        }
    }
    }

    Rectangle {
        background: Colors.cyan;
        height: 25px;

        show-a-button := Button {
            text: "Show A";
            checkable: true;
        }
    }
}

export component Demo {
    Page {}
}
```

## Build slint components as reusable library by Rust crate:

Related components serving a common purpose **must** be packaged as a Cargo workspace member (the **libA** pattern). This section covers the complete setup — library side, client side, and common pitfalls.

### Slint entry file (library)

Create one entry `.slint` file that re-exports everything the client needs:

```slint
// file lib/my_library/ui/my_library.slint
export { MyComponent }    from "components/my_component.slint";
export { MyGlobalLogic }  from "my_global_logic.slint";  // global singleton
```

Internal components import each other by **relative paths** (`import { Foo } from "foo.slint"`). Only the entry file is visible to the client.

Client-side Slint imports use the library name prefixed with `@`:
```slint
import { MyComponent, MyGlobalLogic } from "@my_library";
```

### Library `Cargo.toml`

```toml
[package]
name = "my_library"
version = "0.1.0"
edition = "2021"
links = "my_library"          # required — enables DEP_MY_LIBRARY_* env vars for the client

[dependencies]
slint = { workspace = true }

[build-dependencies]
slint-build = { workspace = true }
```

The `links` key tells Cargo to expose `DEP_MY_LIBRARY_SLINT_*` metadata to any crate that depends on this library. **It must match the crate name (lowercase, hyphens replaced by underscores).**

### Library `build.rs`

```rust
fn main() {
    // as_library + rust_module require the "experimental-module-builds" feature in slint-build.
    // They emit DEP_MY_LIBRARY_SLINT_LIBRARY_* metadata so the client resolves @my_library imports
    // without needing with_library_paths, and ensures the client reuses the library's Rust types
    // (enabling the Global<MainWindow> trait bound in init<T>).
    let config = slint_build::CompilerConfiguration::new()
        .as_library("my_library")
        .rust_module("my_library");
    slint_build::compile_with_config("ui/my_library.slint", config).unwrap();
}
```

### Library `src/lib.rs`

```rust
// IMPORTANT: wrap include_modules!() in pub mod <name> so the client can resolve
// `my_library::my_library::*` — the path that the client's generated code emits.
pub mod my_library {
    slint::include_modules!();
}

use my_library::MyGlobalLogic;

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    // This bound is satisfiable when T = MainWindow because the client's slint::include_modules!()
    // generates impl Global<MainWindow> for my_library::my_library::MyGlobalLogic.
    for<'a> MyGlobalLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<MyGlobalLogic>();
    let ui_weak = ui.as_weak();

    logic.on_some_callback(move || {
        let ui = ui_weak.unwrap();
        let logic = ui.global::<MyGlobalLogic>();
        // implement callback logic here — ALL Rust logic lives in lib.rs, never in client main.rs
        logic.set_some_property(new_value);
    });
}
```

### Client `Cargo.toml` (workspace root)

```toml
[workspace]
members = [".", "lib/my_library"]
resolver = "2"

[workspace.dependencies]
slint = { version = "1.x", features = ["compat-1-0"] }
# experimental-module-builds is required for as_library / rust_module in build.rs
slint-build = { version = "1.x", features = ["experimental-module-builds"] }
my_library = { path = "lib/my_library" }

[package]
name = "my_app"
# ...

[dependencies]
slint = { workspace = true }
my_library = { path = "lib/my_library" }

[build-dependencies]
slint-build = { workspace = true }
```

### Client `build.rs`

```rust
fn main() {
    // No with_library_paths needed — the library emits DEP_MY_LIBRARY_SLINT_* metadata
    // via as_library in its own build.rs, and slint_build picks them up automatically.
    slint_build::compile("ui/main_window.slint").expect("Slint build failed");
}
```

### Client `src/main.rs`

```rust
slint::include_modules!(); // generates: use my_library::my_library; (shadows crate name!)

fn main() {
    let window = MainWindow::new().unwrap();
    // Use :: prefix to unambiguously reference the crate, not the generated module alias.
    ::my_library::init(&window);
    window.run().unwrap();
}
```

### Verification

After a successful build, confirm `init` is called by adding a `println!` to `init()` and running the app. You should see the output before the window appears. Remove the println before committing.

---

### Troubleshooting

| Error | Cause | Fix |
|---|---|---|
| `no method named as_library` / `no method named rust_module` | Feature flag missing | Add `features = ["experimental-module-builds"]` to `slint-build` in workspace deps |
| `error: Error reading ... lib/my_library: Access is denied (os error 5)` | `with_library_paths` points to a **directory** | Remove `with_library_paths` from client build.rs; use `as_library` in library build.rs instead |
| `unresolved import my_library::my_library` | `pub mod my_library` wrapper missing from library `src/lib.rs` | Wrap `slint::include_modules!()` in `pub mod my_library { ... }` |
| `cannot find function init in module my_library` | Generated `use my_library::my_library` shadows the crate name | Call `::my_library::init(&window)` (leading `::` = explicit crate reference) |
| `expected &LibraryComponent, found &MainWindow` (E0308) | `as_library` + `rust_module` not used — client regenerates types inline so `Global<MainWindow>` impl is missing | Add `as_library("my_library").rust_module("my_library")` to library build.rs, add the feature flag, add `pub mod` wrapper |
| Build succeeds but `init` not called | `::my_library::init(&window)` missing from client `main()` | Add the call before `window.run()` |

---

## Use purely Slint reusable library (libC pattern)

When a reusable library contains **only Slint** — design tokens, animation curves, or shared UI primitives with no Rust callback logic — it does **not** need a Rust crate (`Cargo.toml`, `build.rs`, or `src/`). It is just a folder of `.slint` files. Each client registers the folder path in its own `build.rs` using `with_library_paths`.

### When to use
- Design system: color palette, typography scale, spacing tokens, border tokens.
- Animation constants: easing curves, duration tokens.
- Shared UI primitives that have no backend callbacks (e.g., `SwipeArea`, icon components).

### Folder structure

```
lib/
  my_lib/
    tokens.slint          # design tokens
    animations.slint      # easing + duration constants
    my_lib.slint          # entry file — re-exports everything
```

Internal files import each other by relative path. The entry file re-exports the public surface:

```slint
// lib/my_lib/my_lib.slint
export { Tokens }     from "tokens.slint";
export { Animations } from "animations.slint";
```

### Swappable theme files — color sets behind one import line

When the design tokens must support more than one color theme (e.g., day/night, or matching an external palette), do **not** branch on a runtime property inside `tokens.slint`. Instead, give every theme its own file that exports a `Tokens` global with **identical property names and types** but different color values, and let the entry file re-export exactly one of them:

```
lib/my_lib/
  themes/
    theme_default.slint          # export global Tokens { ... current values ... }
    theme_solarized_light.slint  # export global Tokens { ... same properties, new values ... }
  tokens.slint                   # superseded by themes/* — kept only if non-color tokens live here
  my_lib.slint                   # entry file — re-exports the ACTIVE theme
```

```slint
// lib/my_lib/my_lib.slint
export { Tokens } from "themes/theme_default.slint";
// To switch themes: replace "theme_default" with "theme_solarized_light" — one string, one file.
```

Because every theme file implements the same `Tokens` interface, swapping the active theme is a single search-and-replace on the import path — no consuming `.slint` file changes, and no risk of a missing or renamed property breaking the build. Keep typography, spacing, and border-radius tokens (non-color values) out of the theme files — only color sets need duplicating per theme.

### Client `build.rs`

Each client (libA crate or the root app) registers the library path in its `build.rs`:

```rust
fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let library_paths = std::collections::HashMap::from([(
        "my_lib".to_string(),
        manifest_dir.join("../../lib/my_lib/my_lib.slint"),  // path to the entry file
    )]);
    slint_build::compile_with_config(
        "ui/entry.slint",
        slint_build::CompilerConfiguration::new()
            .with_library_paths(library_paths),
    ).unwrap();
}
```

> The path **must point to the entry `.slint` file**, not the folder. Pointing to a directory causes `Access is denied (os error 5)`.

### Importing in `.slint` files

```slint
import { Tokens, Animations } from "@my_lib";
```

### No Rust crate needed — no Cargo.toml, no workspace member

Because the library has no Rust source, it is **not** added to the workspace `members` list and not listed as a `[dependency]`. Only the `build.rs` of each client needs to know the path.

### Animation note — flip effects without Rust

A convincing card-flip animation is achievable in pure Slint using the **width-compression trick**:

1. Animate the card `width` from `parent.width` → `0` over the first half (card "collapses" edge-on).
2. At `width = 0`, switch the displayed content (front ↔ back) using an `if` condition on a `flipped` property.
3. Animate `width` from `0` → `parent.width` over the second half (new face "expands").

```slint
property <bool> flipped: false;

front-face := Rectangle {
    visible: !flipped;
    width: flipped ? 0px : parent.width;
    animate width { duration: 150ms; easing: ease-in; }
    // front content here
}

back-face := Rectangle {
    visible: flipped;
    width: flipped ? parent.width : 0px;
    animate width { duration: 150ms; easing: ease-out; delay: 150ms; }
    // back content here
}
```

> **True perspective-correct 3D rotation** (CSS-style `rotateY` with `perspective`) is not exposed by Slint's property system and would require a custom rendering backend in Rust. Avoid unless the visual difference justifies the complexity — the width-compression trick reads as a 3D flip to users.