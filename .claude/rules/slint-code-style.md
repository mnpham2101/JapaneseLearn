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

## Build slint components as reusable library:
* related components serving a common purpose (e.g., flashcard UI components) **must** be organized into a shared directory (e.g., `ui/components/`) and imported into pages or other components as needed. Use relative paths for imports to maintain clarity and avoid issues with module resolution. Each component file should export a single component that can be easily imported and used in other parts of the application.
* the components should also be built as a Rust module, following the code example: 

```slint
// each slint component is defined in ui/*.slint file
// import other components by relative paths
// file my_component.slint
export global MyLibraryDataModel {};
export component MyComponent { callback my-callback; }

// an entry file should be re-export all components.
// file my_library.slint
export { My Component, MyLibraryDataModel } from "my_component.slint"
```

```rust
// rust build.rs files should be placed at library's root, on the same directory with library Cargo.toml
// file build.rs
use std::path::PathBuf;

fn main() {
    // Add the controls library as a library path so we can import slint files with "@controls/foo.slint"
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // do not add itself to its own library path
    // let library_paths = std::collections::HashMap::from([(
    //     "my_library".to_string(),
    //     manifest_dir.join("../my_library"),
    // )]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths)
        .with_style("cupertino".into());
    // compile the entry file
    slint_build::compile_with_config("ui/my_component.slint", config).expect("Slint build failed");
}

// file src/main.rs:
// define the library as a rust module
pub mod my_library {           
    slint::include_modules!();
    use slint::{ComponentHandle, Model};
    // include init function to init necessary logic, 
    // including defining callback that are declared in slint
    pub fn init<T>(ui: &T)
    // This init function will accept a reference to any component type T,
    // but only under one condition:
    // the component T has knowledge of the global singleton MyLibraryDataModel.
    where
        T: ComponentHandle + 'static,
        for<'a> <'a>: slint::Global<'a, T>,
    {
        // get the instance of the global MyLibraryDataModel to run the callback on_update_my_callback
        let logic = ui.global::<MyLibraryDataModel>();
        // the pointer ui is gone after init returns (heap is MainWindow is still remained)
        // must keep the weak ptr to access MainWindow which is the root component, and move to callback's scope
        let ui_weak = ui.as_weak();

        logic.on_update_my_callback( {
            println!("[INFO] on_update_my_callback.");
        });
    }
}
```

* Cargo.toml of the library must correctly define the that slint and rust code are built as a package:
```
[package]
name = "my_library"
version = "0.1.0"
edition = "2024"
links = "my_library"

[dependencies]
slint = { workspace = true }

[build-dependencies]
slint-build = { workspace = true }
```

* The client application must be configured to use the library correctly. Starting with its root Cargo.toml:
```
[dependencies]
my_library = { // path to where the libary is  }
```
* main function of main.rs should call init function defined in the library to register any callback
```rust
use my_library::my_libary;
fn main(){
   let ui = MainWindow::new().unwrap();
   // call init function defined in my_library/src/lib.rs
   init(&ui);    
}
```
* the global data model in the library should be made available to the global context to be used by client app
```slint
import { MyLibraryModel } from @my_library
```
* note that `my_library` is the name of the Slint library, defined by its Cargo.toml file, and should be used in the import section now. 