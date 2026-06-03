---
agent: speckit.clarify
---
# Design common button:
* check slint document for behavior of `enabled`, `checked`, `checkable`, `pressed` properties and add same properties and logic to common_button. Keep the name, and comments:
```
// button could be disable. Callback can only be invoked when enabled is true.
in-out property <bool> enabled <=> ta.enabled: true;
// if the button is clicked, button state changes. Useful for toggle button, in flashcard
in-out property <bool> checked: falsed;
// if the button is checkable, it can be checked or unchecked when clicked. Otherwise, it behaves like a normal button without checked state. Usefull for mutual exclusive button group, in flashcard stack list.
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

# Prefer declarative property binding over imperative callbacks:
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

# Mutually exclusive button group pattern:
* For a group of buttons where only one can be active (e.g., flashcard stack selection), use `checkable: true` on each button and bind their `checked` properties to a shared state that tracks the selected stack. This allows Slint to manage the exclusivity without manual state toggling in callbacks. Follow the pattern:
```slint
export component Page {
    property <int> current-button-index: 0;

    HorrizontalView {
      for item in stackList: CommonButton {
          text: item.name;
          checkable: true;
          checked: (current-button-index == self.index) ? true : false;  // bind the checked state to the current index of the parent component
          clicked => { current-button-index = self.index; }   // set current index on click, Slint will handle the checked state based on the binding
      }
    }
}
``` 

# Vertically stacked up components pattern:
* To stack components vertically, use a `VerticalLayout` or `VerticalBox` container. This ensures consistent spacing and alignment without manual positioning. 
* To enable displaying/hidding components, use property `LayoutAlignment.end` so that the components can appears to be stacked up / removed from bottom to top. 
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